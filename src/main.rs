use mpd::{Client, Song, State, Status};
use notify_rust::{Notification, Timeout};
use sedregex::find_and_replace;
use std::fmt;

// Struct and impl for printing the state as a string
struct PlayState {
    sta: State,
}

impl fmt::Display for PlayState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.sta)
    }
}

// Borrowed from JakeStanger, I don't usually do this, but I thought I was going to go insane
// If you come across this and you're mad, I sincerely apologize and will remove/replace this
fn format_time(time: i64) -> String {
    let seconds = (time as f64 % 60.0).round();
    let minutes = ((time as f64 % 3600.0) / 60.0).round();

    format!("{:0>2}:{:0>2}", minutes, seconds)
}

// format/title/artist/album/date/genre -> Strings
fn info(c: &mut Client) -> (String, String, String, String, String, String) {
    let song: Song = c.currentsong().unwrap().unwrap();
    let fil = song.file;
    let format = find_and_replace(&fil, &["s/.*\\.//"]).unwrap().to_string();
    let na = "N/A".to_string();
    let tit = song.title.as_ref().unwrap().to_string();
    let art = song.tags.get("Artist").unwrap_or(&na).to_string();
    let alb = song.tags.get("Album").unwrap_or(&na).to_string();
    let dat = song.tags.get("Date").unwrap_or(&na).to_string();
    let gen = song.tags.get("Genre").unwrap_or(&na).to_string();
    (format, tit, art, alb, dat, gen)
}

// elapsed/duration and bitrate -> Strings
fn info_extended(status: &mut Status) -> (String, String, String) {
    let elap = status.elapsed.unwrap().num_seconds();
    let elapsed = format_time(elap);
    let dur = status.duration.unwrap().num_seconds();
    let duration = format_time(dur);
    let bitrate = status.bitrate.unwrap().to_string();
    let bitrate = [bitrate, " kbps".to_string()].concat();
    (elapsed, duration, bitrate)
}

fn main() {
    let mut c = Client::connect("127.0.0.1:6600").unwrap();
    let mut status: Status = c.status().unwrap();
    let (format, tit, art, alb, dat, gen) = info(&mut c);
    let (elapsed, duration, bitrate) = info_extended(&mut status);
    let stat = status.state;
    let state = PlayState { sta: stat };
    let info = [format, " @ ".to_string(), bitrate].concat();
    // elapsed/duration [state]
    // title [format @ bitrate kbps]
    // album [date]
    // artist
    // genre
    // ^^ Output of `msg`
    let msg = [elapsed, "/".to_string(), duration, " [".to_string(),
        state.to_string(), "]\n".to_string(), tit, " [".to_string(),
        info, "]\n".to_string(), alb, " [".to_string(), dat,
        "]\n".to_string(), art, "\n".to_string(), gen].concat();
    Notification::new()
        .summary(&msg)
        .icon("/tmp/cover.png") // Cover art should be wrote to `/tmp/cover.png`
        .timeout(Timeout::Milliseconds(6000)) // Notification closes in 6s
        .show()
        .unwrap();
}
