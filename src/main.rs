use std::{
    collections::{HashMap, HashSet},
    fs,
    io::{self, ErrorKind},
    path::{Path, PathBuf},
    process,
};
use clap::Parser;
use encoding_rs::{Encoding, UTF_8, GBK, GB18030, BIG5, UTF_16LE, UTF_16BE};
//use walkdir::WalkDir;
use regex::Regex;


// 命令行参数结构
#[derive(Parser, Debug)]
#[command(version, about = "检测代码仓库中是否存在中文字符")]
struct Args {
    /// 要扫描的目录路径
    #[arg(default_value = ".")]
    path: PathBuf,

    /// 要检查的文件扩展名（逗号分隔）
    #[arg(
        short, 
        long,
        value_delimiter = ',',
        default_value = "rs,cpp,java,py,js,go,ts,html,css,h,md,cc"
        // Define the file extensions to be detected (case-insensitive)
    )]
    extensions: Vec<String>,
}




// Define the regular expression for the Chinese Unicode range (compact format)
const CHINESE_PATTERN: &str = r"[\u4e00-\u9fff]";

fn decode_with_encodings(data: &[u8]) -> Result<String, &'static str> {
    // Enhanced encoding detection logic (including BOM detection)
    if data.starts_with(&[0xEF, 0xBB, 0xBF]) { // UTF-8 BOM
        return Ok(UTF_8.decode(&data[3..]).0.into_owned());
    } else if data.starts_with(&[0xFF, 0xFE]) { // UTF-16LE BOM
        return Ok(UTF_16LE.decode(&data[2..]).0.into_owned());
    } else if data.starts_with(&[0xFE, 0xFF]) { // UTF-16BE BOM
        return Ok(UTF_16BE.decode(&data[2..]).0.into_owned());
    }

    // Optimized order of encoding attempts
    let encodings: Vec<&'static Encoding> = vec![
        UTF_8,      // Most commonly used
        GBK,        // Simplified Chinese
        GB18030,    // GBK extension
        BIG5,       // Traditional Chinese
        UTF_16LE,   // UTF-16 little endian
        UTF_16BE,   // UTF-16 big endian
    ];

    for &enc in &encodings {
        let (cow, _, had_errors) = enc.decode(data);
        if !had_errors {
            return Ok(cow.into_owned());
        }
    }
    
    Err("Failed to decode with all attempted encodings")
}

fn check_chinese(path: &Path) -> Vec<usize> {
    let data = match fs::read(path) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("Failed to read file {}: {}", path.display(), e);
            return Vec::new();
        }
    };

    let text = match decode_with_encodings(&data) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Failed to decode {}: {}", path.display(), e);
            return Vec::new();
        }
    };

    // Pre-compile the regular expression (add surrogate pair support)
    let re = Regex::new(CHINESE_PATTERN).expect("Invalid regular expression");
    // Handle different line break formats (support \r\n and \n)
    text.split('\n')
        .enumerate()
        .filter_map(|(i, line)| {
            if re.find(line).is_some() {
                Some(i + 1) // Line numbers start from 1
            } else {
                None
            }
        })
        .collect()
}

fn traverse_dir(dir: &Path, extensions: &[String]) -> io::Result<HashMap<PathBuf, Vec<usize>>>  {
    let mut results = HashMap::new();
    let walker = ignore::WalkBuilder::new(dir)
        .standard_filters(true)  
        .hidden(false)           
        .git_ignore(true)        
        .git_global(false)       
        .git_exclude(false)      
        .build();

    for entry in walker.into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension()
                .and_then(|e| e.to_str())
                .map(|e| e.to_lowercase()) 
            {
                if extensions.iter().any(|code_ext| ext.eq(code_ext) ) {
                    let lines = check_chinese(path);
                    if !lines.is_empty() {
                        results.insert(path.to_path_buf(), lines);
                    }
                }
            }
        }
    }

    Ok(results)
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    
 
    let mut extensions =  args.extensions.clone();

    extensions = extensions.iter()
        .map(|e| e.to_lowercase())
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();


    
    let dir =&args.path;
    let dir_path = Path::new(&dir).canonicalize().map_err(|e| {
        io::Error::new(
            ErrorKind::NotFound,
            format!("Directory does not exist: {:?} ({})", dir, e)
        )
    })?;

    println!("Scanning directory: {}", dir_path.display());
    let results = traverse_dir(&args.path, &extensions)?;
  
    if !results.is_empty() {
        println!("\nChinese characters found in the following files:");
        for (path, lines) in &results {
            let lines_str: Vec<String> = lines.iter().map(|n| n.to_string()).collect();
            println!("{} : Line numbers [{}]", path.display(), lines_str.join(", "));
        }
        
        println!("\nError: Chinese characters are included in the code files");
        process::exit(1);
    } else {
        println!("\nCheck completed: No Chinese characters found in the code files");
        process::exit(0);
    }
}