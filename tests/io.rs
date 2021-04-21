#![cfg(feature = "default")]
use lofty::{DetermineFrom, MimeType, Picture, Tag};

macro_rules! full_test {
	($function:ident, $file:expr) => {
		#[test]
		fn $function() {
			println!("-- Adding tags --");
			add_tags!($file);
			println!("-- Removing tags --");
			remove_tags!($file);
		}
	};
}

macro_rules! add_tags {
	($file:expr) => {
		println!("Reading file");
		let mut tag = Tag::default()
			.read_from_path($file, DetermineFrom::Signature)
			.unwrap();

		println!("Setting title");
		tag.set_title("foo title");
		assert_eq!(tag.title(), Some("foo title"));

		println!("Setting artist");
		tag.set_artist("foo artist");
		assert_eq!(tag.artist_str(), Some("foo artist"));

		println!("Setting year");
		tag.set_year(2020);
		assert_eq!(tag.year(), Some(2020));

		println!("Setting album title");
		tag.set_album_title("foo album title");
		assert_eq!(tag.album_title(), Some("foo album title"));

		println!("Setting album artists");
		tag.set_album_artist("foo album artist");
		assert_eq!(tag.album_artists_vec(), Some(vec!["foo album artist"]));

		// TODO
		// let cover = Picture {
		// 	mime_type: MimeType::Jpeg,
		// 	data: &vec![0u8; 10],
		// };
		//
		// tags.set_album_cover(cover.clone());
		// assert_eq!(tags.album_cover(), Some(cover));

		println!("Writing");
		tag.write_to_path($file).unwrap();
	};
}

macro_rules! remove_tags {
	($file:expr) => {
		println!("Reading file");
		let mut tag = Tag::default()
			.read_from_path($file, DetermineFrom::Signature)
			.unwrap();

		println!("Checking title");
		assert_eq!(tag.title(), Some("foo title"));

		println!("Removing title");
		tag.remove_title();
		assert!(tag.title().is_none());
		tag.remove_title(); // should not panic

		println!("Removing artist");
		tag.remove_artist();
		assert!(tag.artist_str().is_none());
		tag.remove_artist();

		println!("Removing year");
		tag.remove_year();
		assert!(tag.year().is_none());
		tag.remove_year();

		println!("Removing album title");
		tag.remove_album_title();
		assert!(tag.album_title().is_none());
		tag.remove_album_title();

		println!("Removing album artists");
		tag.remove_album_artists();
		assert!(tag.album_artists_vec().is_none());
		tag.remove_album_artists();

		// TODO
		// tags.remove_album_cover();
		// assert!(tags.album_cover().is_none());
		// tags.remove_album_cover();

		println!("Writing");
		tag.write_to_path($file).unwrap();
	};
}

full_test!(test_ape, "tests/assets/a.ape");
full_test!(test_m4a, "tests/assets/a.m4a");
full_test!(test_mp3, "tests/assets/a.mp3");
full_test!(test_wav, "tests/assets/a.wav");

// Vorbis comments
full_test!(test_flac, "tests/assets/a.flac");
full_test!(test_ogg, "tests/assets/a.ogg");
full_test!(test_opus, "tests/assets/a.opus");
