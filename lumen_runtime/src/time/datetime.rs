cfg_if::cfg_if! {
  if #[cfg(all(target_arch = "wasm32", feature = "time_web_sys"))] {
     mod web_sys;
     pub use self::web_sys::*;
  } else {
     mod std;
     pub use self::std::*;
  }
}

pub fn utc_now() -> [usize; 6] {
    get_utc_now()
}

pub fn local_now() -> [usize; 6] {
    get_local_now()
}

pub fn local_datetime_to_utc_datetime(datetime: [usize; 6]) -> [usize; 6] {
    convert_local_datetime_to_utc_datetime(datetime)
}

pub fn utc_datetime_to_local_datetime(datetime: [usize; 6]) -> [usize; 6] {
    convert_utc_datetime_to_local_datetime(datetime)
}

pub fn local_date() -> [usize; 3] {
    let datetime: [usize; 6] = get_local_now();
    [datetime[0], datetime[1], datetime[2]]
}

pub fn local_time() -> [usize; 3] {
    let datetime: [usize; 6] = get_local_now();
    [datetime[3], datetime[4], datetime[5]]
}
