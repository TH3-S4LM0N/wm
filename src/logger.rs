use {
    std::{
        path::Path,
        fs::{write, File},
        fmt
    },
    chrono::Local
};

pub fn log<P>(msg: &str, logfile_path: &P)
where P: AsRef<Path>
{
    write(logfile_path, &format!("{} - {}\n", Local::now().format("%H:%M:%S"),  msg)).expect("Failed to write logfile");
}

pub fn log_err<P>(msg: &str, logfile_path: &P, err: &dyn fmt::Debug) -> !
where P: AsRef<Path>
{
    let err_msg = format!("{} - {}: {:?}", Local::now().format("%H:%M:%S"), msg, err);

    write(logfile_path, &err_msg).expect("Failed to write logfile");

    panic!("{}", &err_msg);
}

pub fn log_result<P>(msg: &str, err: &dyn fmt::Debug, logfile_path: &P) -> !
where P: AsRef<Path>
{
    log_err(msg, logfile_path, err);
}
pub fn log_opt<P>(msg: &str, logfile_path: &P) -> !
where P: AsRef<Path>
{
    log(msg, logfile_path);
    panic!("{msg}");
}


pub fn init_logger<P>(logfile_path: &P)
where P: AsRef<Path>
{
    File::create(&logfile_path)
        .expect("Failed to create logfile!");

    write(
        &logfile_path,
        &format!(
            "{} :: Created trout logfile",k
            Local::now().format("%H:%M:%S")
        ),
    )
    .expect("Failed to write logfile!k");
}

// trait to add a log method to replace `.expect` for Result<T, E>
pub trait ResultExt<T, E> {
    fn log<P>(self, msg: &str, logfile_path: &P) -> T
    where
        E: fmt::Debug,
        P: AsRef<Path>;
}
impl<T, E> ResultExt<T, E> for Result<T, E> {
    fn log<P>(self, msg: &str, logfile_path: &P) -> T
    where
        E: fmt::Debug,
        P: AsRef<Path>,
    {
        match self {
            Ok(t) => t,
            Err(e) => log_result(msg, &e, &logfile_path),
        }
    }
}

// trait to add log method to replace `.expect` for Option<T>
pub trait OptionExt<T> {
    fn log<P>(self, msg: &str, logfile_path: P) -> T
    where P: AsRef<Path>;
}
impl<T> OptionExt<T> for Option<T> {
    fn log<P>(self, msg: &str, logfile_path: P) -> T
    where P: AsRef<Path>
    {
        match self {
            Some(val) => val,
            None => log_opt(msg, &logfile_path)
        }
    }
}