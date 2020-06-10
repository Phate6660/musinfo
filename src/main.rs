use mpd::{Client, Song, State, Status};
use notify_rust::{Notification, Timeout};
use std::fmt;

// Struct and impl for printing the fucking state as a fucking string
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

// title/artist/album/date/genre -> strings
fn info(c: &mut Client) -> (String, String, String, String, String) {
    let song: Song = c.currentsong().unwrap().unwrap();
    let tit = song.title.as_ref().unwrap();
    let art = song.tags.get("Artist").unwrap();
    let alb = song.tags.get("Album").unwrap();
    let dat = song.tags.get("Date").unwrap();
    let gen = song.tags.get("Genre").unwrap();

    (
        tit.to_string(),
        art.to_string(),
        alb.to_string(),
        dat.to_string(),
        gen.to_string(),
    )
}

// elapsed/duration and bitrate -> string
fn info_extended(status: &mut Status) -> (String, String, String) {
    let elap = status.elapsed.unwrap().num_seconds();
    let elapsed = format_time(elap);
    let dur = status.duration.unwrap().num_seconds();
    let duration = format_time(dur);
    let bitrate = status.bitrate.unwrap().to_string();
    let bitrate = bitrate + &" kbps".to_string();
    (elapsed, duration, bitrate)
}

fn main() {
    let mut c = Client::connect("127.0.0.1:6600").unwrap();
    let mut status: Status = c.status().unwrap();
    let (tit, art, alb, dat, gen) = info(&mut c);
    let (elapsed, duration, bitrate) = info_extended(&mut status);
    let stat = status.state;
    let state = PlayState { sta: stat };
    // elapsed/duration [state]
    // title [bitrate]
    // album [date]
    // artist
    // genre
    //
    // ^^ Output of `msg` (without the extra space at the bottom)
    let msg = elapsed
        + &"/".to_string()
        + &duration
        + &" [".to_string()
        + &state.to_string()
        + &"]\n".to_string()
        + &tit
        + &" [".to_string()
        + &bitrate
        + &"]\n".to_string()
        + &alb
        + &" [".to_string()
        + &dat
        + &"]\n".to_string()
        + &art
        + &"\n".to_string()
        + &gen;
    Notification::new()
        .summary(&msg)
        .icon("/tmp/cover.png") // Cover art should be wrote to `/tmp/cover.png`
        .timeout(Timeout::Milliseconds(6000)) // Notification closes in 6s
        .show()
        .unwrap();
}
