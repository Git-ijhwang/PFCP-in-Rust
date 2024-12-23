use std::fs;
use chrono::Local;
use std::path::Path;


fn check_and_roll_log(log_filename: &str) -> std::io::Result<fs::File> {
    let max_size = 10 * 1024 * 1024; // 10MB
    let log_path = Path::new(log_filename);

    if let Ok(file_metadata) = fs::metadata(log_path) {
        if file_metadata.len() > max_size {
            let new_filename = format!("{}-{}", log_filename, Local::now().format("%Y-%m-%d-%H-%M-%S"));
            println!("Log file size exceeded 10MB, rolling to: {}", new_filename);
            return Ok(fs::File::create(new_filename)?);
        }
    }
    
    fs::File::create(log_filename)
}


fn setup_logger(log: &str) -> Result<(), fern::InitError> {

    let log_dir = "logs";
    if !fs::metadata(log_dir).is_ok() {
        fs::create_dir(log_dir).unwrap(); // create directory for log file
    }

    let log_filename = format!("{}/logging_{}.log", log_dir, Local::now().format("%Y-%m-%d"));

    // 파일 롤링 처리: 로그 파일 크기를 확인하여 10MB 초과 시 새로운 파일 생성
    let log_file = check_and_roll_log(&log_filename).unwrap();

    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                // humantime::format_rfc3339_seconds(SystemTime::now()),
                Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        // .chain(std::io::stdout())
        // .chain(fern::log_file(log_filename)?)
        .chain(log_file)
        .apply()?;

    Ok(())
}