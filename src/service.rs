//! UI-independent YouTube services and lightweight local persistence.
use rustypipe::{client::RustyPipe, model::{richtext::ToPlaintext, VideoItem, YouTubeItem}, param::search_filter::{ItemType, Length, Order, SearchFilter, UploadDate}};
use serde::{Deserialize, Serialize};
use std::{fs, path::{Path, PathBuf}, process::Command};

fn app_data_dir() -> PathBuf {
    std::env::var_os("APPDATA").map(PathBuf::from).unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))).join("youtube-desktop")
}
fn ensure_dir(path: &Path) -> Result<(), String> { fs::create_dir_all(path).map_err(|e| format!("{}: {}", path.display(), e)) }
fn build_client() -> Result<RustyPipe, String> {
    let cache = app_data_dir().join("cache"); ensure_dir(&cache)?;
    RustyPipe::builder().storage_dir(cache).build().map_err(|e| e.to_string())
}
fn read_json<T: for<'a> Deserialize<'a> + Default>(name: &str) -> T {
    fs::read(app_data_dir().join(name)).ok().and_then(|b| serde_json::from_slice(&b).ok()).unwrap_or_default()
}
fn write_json<T: Serialize>(name: &str, value: &T) -> Result<(), String> {
    let dir=app_data_dir(); ensure_dir(&dir)?;
    fs::write(dir.join(name), serde_json::to_vec_pretty(value).map_err(|e|e.to_string())?).map_err(|e|e.to_string())
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SearchOptions { pub item_type:usize, pub date:usize, pub duration:usize, pub sort:usize }
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SearchResultItem { pub kind:String, pub title:String, pub subtitle:String, pub id:String, pub url:String, pub thumbnail_url:String }
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct DetailData { pub heading:String, pub subheading:String, pub description:String, pub id:String, pub url:String, pub thumbnail_url:String, pub items:Vec<SearchResultItem> }

fn video_item(v: VideoItem) -> SearchResultItem {
    let channel=v.channel.as_ref().map(|c|c.name.clone()).unwrap_or_else(||"Unknown channel".into());
    let mut subtitle=channel;
    if let Some(n)=v.view_count { subtitle.push_str(&format!(" · {} views",n)); }
    if let Some(n)=v.duration { subtitle.push_str(&format!(" · {} sec",n)); }
    SearchResultItem { kind:"VIDEO".into(), title:v.name, subtitle, id:v.id.clone(), url:format!("https://www.youtube.com/watch?v={}",v.id), thumbnail_url:v.thumbnail.into_iter().last().map(|t|t.url).unwrap_or_default() }
}
fn from_item(item:YouTubeItem)->SearchResultItem {
    match item {
        YouTubeItem::Video(v)=>video_item(v),
        YouTubeItem::Channel(c)=>SearchResultItem { kind:"CHANNEL".into(),title:c.name,subtitle:format!("{} subscribers",c.subscriber_count.unwrap_or(0)),id:c.id.clone(),url:format!("https://www.youtube.com/channel/{}",c.id),thumbnail_url:c.avatar.into_iter().last().map(|t|t.url).unwrap_or_default() },
        YouTubeItem::Playlist(p)=>SearchResultItem { kind:"PLAYLIST".into(),title:p.name,subtitle:format!("{} videos",p.video_count.unwrap_or(0)),id:p.id.clone(),url:format!("https://www.youtube.com/playlist?list={}",p.id),thumbnail_url:p.thumbnail.into_iter().last().map(|t|t.url).unwrap_or_default() },
    }
}
pub async fn search(query:String, o:SearchOptions)->Result<Vec<SearchResultItem>,String>{
    let client=build_client()?;
    let f=SearchFilter::new()
      .date_opt(match o.date{1=>Some(UploadDate::Hour),2=>Some(UploadDate::Day),3=>Some(UploadDate::Week),4=>Some(UploadDate::Month),5=>Some(UploadDate::Year),_=>None})
      .sort_opt(match o.sort{1=>Some(Order::Date),2=>Some(Order::Views),3=>Some(Order::Rating),_=>None})
      .length_opt(match o.duration{1=>Some(Length::Short),2=>Some(Length::Medium),3=>Some(Length::Long),_=>None})
      .item_type_opt(match o.item_type{1=>Some(ItemType::Video),2=>Some(ItemType::Playlist),3=>Some(ItemType::Channel),_=>None});
    let r=client.query().search_filter(query,&f).await.map_err(|e|e.to_string())?;
    Ok(r.items.items.into_iter().map(from_item).collect())
}
pub async fn video_detail(id:String)->Result<DetailData,String>{
    let c=build_client()?;let q=c.query();let (player,details)=tokio::join!(q.player(&id),q.video_details(&id));
    let player=player.map_err(|e|e.to_string())?;let d=details.map_err(|e|e.to_string())?;
    let duration=player.details.duration.map(|n|format!("{} seconds",n)).unwrap_or_else(||"Duration unavailable".into());
    Ok(DetailData{heading:player.details.name.unwrap_or_else(||"YouTube video".into()),subheading:format!("{} · {} views · {}",d.channel.name,d.view_count,duration),description:d.description.to_plaintext(),id:id.clone(),url:format!("https://www.youtube.com/watch?v={}",id),thumbnail_url:player.details.thumbnail.iter().last().map(|t|t.url.clone()).unwrap_or_default(),items:Vec::new()})
}
pub async fn channel_detail(id:String)->Result<DetailData,String>{
    let c=build_client()?;let q=c.query();let (info,artist,videos,playlists)=tokio::join!(q.channel_info(&id),q.music_artist(&id,false),q.channel_videos(&id),q.channel_playlists(&id));
    let info=info.map_err(|e|e.to_string())?;let name=artist.map(|a|a.name).unwrap_or_else(|_|format!("Channel {}",id));let mut items=Vec::new();
    if let Ok(v)=videos {items.extend(v.content.items.into_iter().map(video_item));}
    if let Ok(v)=playlists {items.extend(v.content.items.into_iter().map(|p|SearchResultItem{kind:"PLAYLIST".into(),title:p.name,subtitle:format!("{} videos",p.video_count.unwrap_or(0)),id:p.id.clone(),url:format!("https://www.youtube.com/playlist?list={}",p.id),thumbnail_url:p.thumbnail.into_iter().last().map(|t|t.url).unwrap_or_default()}));}
    Ok(DetailData{heading:name,subheading:format!("{} subscribers · {} views",info.subscriber_count.unwrap_or(0),info.view_count.unwrap_or(0)),description:info.description,id:id.clone(),url:format!("https://www.youtube.com/channel/{}",id),thumbnail_url:String::new(),items})
}
pub async fn playlist_detail(id:String)->Result<DetailData,String>{
    let c=build_client()?;let p=c.query().playlist(&id).await.map_err(|e|e.to_string())?;
    let author=p.channel.as_ref().map(|x|x.name.clone()).unwrap_or_else(||"Unknown channel".into());
    let items=p.videos.items.into_iter().map(|v|SearchResultItem{kind:"VIDEO".into(),title:v.name,subtitle:author.clone(),id:v.id.clone(),url:format!("https://www.youtube.com/watch?v={}",v.id),thumbnail_url:v.thumbnail.into_iter().last().map(|t|t.url).unwrap_or_default()}).collect();
    Ok(DetailData{heading:p.name,subheading:format!("{} · {} videos",author,p.video_count),description:p.description.map(|x|x.to_plaintext()).unwrap_or_default(),id:id.clone(),url:format!("https://www.youtube.com/playlist?list={}",id),thumbnail_url:p.thumbnail.into_iter().last().map(|t|t.url).unwrap_or_default(),items})
}
pub async fn trending()->Result<Vec<SearchResultItem>,String>{let c=build_client()?;let r=c.query().trending().await.map_err(|e|e.to_string())?;Ok(r.into_iter().map(video_item).collect())}

pub fn load_search_history()->Vec<String>{let mut v:Vec<String>=read_json("search-history.json");v.truncate(20);v}
pub fn save_search_history(q:&str)->Result<(),String>{let q=q.trim();if q.is_empty(){return Ok(())}let mut v=load_search_history();v.retain(|x|!x.eq_ignore_ascii_case(q));v.insert(0,q.into());v.truncate(20);write_json("search-history.json",&v)}
#[derive(Clone,Debug,Serialize,Deserialize,Default)] pub struct WatchEntry{pub title:String,pub id:String,pub url:String,pub watched_at:u64}
pub fn add_watch_history(i:&SearchResultItem)->Result<(),String>{let mut v:Vec<WatchEntry>=read_json("watch-history.json");v.retain(|x|x.id!=i.id);v.insert(0,WatchEntry{title:i.title.clone(),id:i.id.clone(),url:i.url.clone(),watched_at:std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs()});v.truncate(100);write_json("watch-history.json",&v)}
pub fn watch_history()->Vec<SearchResultItem>{let v:Vec<WatchEntry>=read_json("watch-history.json");v.into_iter().map(|x|SearchResultItem{kind:"VIDEO".into(),title:x.title,subtitle:format!("Watched at {}",x.watched_at),id:x.id,url:x.url,thumbnail_url:String::new()}).collect()}
#[derive(Clone,Debug,Serialize,Deserialize,Default)] pub struct Subscription{pub id:String,pub name:String}
pub fn subscriptions()->Vec<Subscription>{read_json("subscriptions.json")}
pub async fn subscribe(id:String)->Result<(),String>{let c=build_client()?;let q=c.query();let (info,artist)=tokio::join!(q.channel_info(&id),q.music_artist(&id,false));info.map_err(|e|e.to_string())?;let name=artist.map(|a|a.name).unwrap_or_else(|_|format!("Channel {}",id));let mut v=subscriptions();if !v.iter().any(|x|x.id==id){v.push(Subscription{id,name});}write_json("subscriptions.json",&v)}
pub fn unsubscribe(id:&str)->Result<(),String>{let mut v=subscriptions();v.retain(|x|x.id!=id);write_json("subscriptions.json",&v)}
pub fn subscription_items()->Vec<SearchResultItem>{subscriptions().into_iter().map(|x|SearchResultItem{kind:"CHANNEL".into(),title:x.name,subtitle:"Subscribed channel · double-click to browse".into(),id:x.id.clone(),url:format!("https://www.youtube.com/channel/{}",x.id),thumbnail_url:String::new()}).collect()}
pub fn download_dir()->PathBuf{let p=app_data_dir().join("downloads");let _=ensure_dir(&p);p}
pub fn offline_library()->Vec<SearchResultItem>{let mut v=Vec::new();if let Ok(entries)=fs::read_dir(download_dir()){for e in entries.flatten(){let p=e.path();if p.is_file(){let name=p.file_name().unwrap_or_default().to_string_lossy().into_owned();v.push(SearchResultItem{kind:"LOCAL FILE".into(),title:name,subtitle:p.display().to_string(),id:p.to_string_lossy().into_owned(),url:p.to_string_lossy().into_owned(),thumbnail_url:String::new()});}}}v}
pub fn launch_mpv(i:&SearchResultItem)->Result<(),String>{Command::new("mpv").arg("--force-window=yes").arg(&i.url).spawn().map(|_|()).map_err(|e|format!("MPV launch failed. Install mpv and add mpv.exe to PATH. {}",e))}
pub fn launch_ytdlp(i:&SearchResultItem)->Result<(),String>{let dir=download_dir();Command::new("yt-dlp").arg("-o").arg(dir.join("%(title)s.%(ext)s")).arg(&i.url).spawn().map(|_|()).map_err(|e|format!("yt-dlp launch failed. Install yt-dlp.exe and add it to PATH. {}",e))}
