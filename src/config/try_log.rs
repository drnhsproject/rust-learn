#[cfg(test)]
mod tests_log {
    use log::{error, info, warn, debug, trace};

    #[test]
    fn test_try_log() {
        env_logger::init();
        
        error!("This is an error message");
        warn!("This is a warning message");
        info!("This is an info message");
        debug!("This is a debug message");
        trace!("This is a trace message");
    }

    #[test]
    fn test_try_log_complex() {
        let file_config: String = "src/config/log_config.yaml".to_string();  
        log4rs::init_file(file_config, Default::default()).unwrap();
        
        error!("This is an error message");
        warn!("This is a warning message");
        info!("This is an info message");
        debug!("This is a debug message");
        trace!("This is a trace message");
    }
}

#[cfg(test)]
mod tests_log_2 {
    use log::{error, info, warn, debug, trace};

    #[test]
    fn test_try_log_complex_2() {
        let file_config: String = "src/config/log_config.yaml".to_string();  
        log4rs::init_file(file_config, Default::default()).unwrap();
        
        error!("This is an error message");
        warn!("This is a warning message");
        info!("This is an info message");
        debug!("This is a debug message");
        trace!("This is a trace message");
    }
}