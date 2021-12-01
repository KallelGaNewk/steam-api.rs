use chrono::prelude::DateTime;
use chrono::Utc;
use simpleini::Ini;
use std::time::{Duration, UNIX_EPOCH};
use std::{fs::File, io::Write, process::exit};
use steam_api;

fn main() {
    let ini = parse_ini();
    let api_key = ini.get("api_key").unwrap();
    let steam_id = ini.get("steam_id").unwrap();

    if api_key.is_empty() || steam_id.is_empty() {
        eprintln!("api_key or steam_id in config.ini is empty.");
        exit(1);
    }

    let user = &match steam_api::get_profile_info(&vec![steam_id], api_key) {
        Ok(users) => users,
        Err(e) => {
            eprintln!("Failed to get profile info: {:?}", e.to_string());
            exit(1);
        }
    }
    .user[0];

    println!(
        r#"
    SteamID64 ...... {}
    Username ....... {}
    Level .......... {}
    URL ............ {}
    VAC Bans ....... {}
    Last Logoff .... {}
    "#,
        user.steamid,
        user.personaname,
        user.player_level,
        user.profileurl,
        user.NumberOfVACBans,
        DateTime::<Utc>::from(
            UNIX_EPOCH + Duration::from_secs(user.lastlogoff.try_into().unwrap())
        )
        .format("%Y-%m-%d %H:%M:%S")
        .to_string()
    );
}

fn parse_ini() -> Ini {
    match Ini::from_file("config.ini") {
        Ok(ini) => ini,
        Err(e) => {
            eprintln!("Failed to read INI: {:?}", e.to_string());

            if e.to_string().contains("os error 2") {
                write_file("config.ini", b"; find the api key here: https://steamcommunity.com/dev/apikey\napi_key=\n\n; find your SteamID64 here: https://steamid.io/\nsteam_id=76561199112327707\n");
                println!("config.ini created!");
                exit(0);
            }

            exit(1);
        }
    }
}

fn write_file(name: &str, content: &[u8]) {
    let mut file = File::create(name).unwrap();
    file.write_all(content).ok();

    ()
}
