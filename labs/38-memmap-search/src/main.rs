//! # High-Performance Search with Memory-Mapped Files - Demo
//! 
//! This binary creates a large test file and then compares the performance
//! of searching for a pattern using standard file I/O versus a memory map.
//! 
//! Run in release mode for accurate timings:
//! cargo run -p memmap-search --release

use memmap_search::solution::{
    create_test_file,
    search_with_mmap,
    search_with_read,
    parallel_search_with_mmap
};
use std::time::Instant;
use tempfile::Builder;

fn main() -> std::io::Result<()> {
    println!("=== Memory-Map Search Demo ===\n");

    // Create a temporary file to work with.
    let temp_dir = Builder::new().prefix("memmap_demo").tempdir()?;
    let file_path = temp_dir.path().join("large_test_file.txt");
    let file_size_mb = 100; // 100 MB
    let pattern = "RUSTacean";

    println!("Creating a {} MB test file at: {:?}", file_size_mb, file_path);
    create_test_file(&file_path, file_size_mb, pattern)?;
    println!("Test file created.\n");

    println!(
        "Searching for the pattern \"{}\" in the file...\n",
        pattern
    );

    // ============================================================================ 
    // DEMO 1: Search with standard BufReader
    // ============================================================================ 
    println!("1. Searching with standard I/O (BufReader)...");
    let start_read = Instant::now();
    let count_read = search_with_read(&file_path, pattern).unwrap();
    let duration_read = start_read.elapsed();
    println!("   -> Found {} occurrences.", count_read);
    println!("   -> Time taken: {:?}", duration_read);
    println!();

    // ============================================================================ 
    // DEMO 2: Search with Memory-Mapped File
    // ============================================================================ 
    println!("2. Searching with memory-mapping...");
    let start_mmap = Instant::now();
    let count_mmap = search_with_mmap(&file_path, pattern).unwrap();
    let duration_mmap = start_mmap.elapsed();
    println!("   -> Found {} occurrences.", count_mmap);
    println!("   -> Time taken: {:?}", duration_mmap);
    println!();
    
    // ============================================================================ 
    // DEMO 3: Parallel Search with Memory-Mapped File
    // ============================================================================ 
    println!("3. Searching with memory-mapping and parallel processing (Rayon)...");
    let start_par_mmap = Instant::now();
    let count_par_mmap = parallel_search_with_mmap(&file_path, pattern).unwrap();
    let duration_par_mmap = start_par_mmap.elapsed();
    println!("   -> Found {} occurrences.", count_par_mmap);
    println!("   -> Time taken: {:?}", duration_par_mmap);
    println!();


    // ============================================================================ 
    // Comparison
    // ============================================================================ 
    assert_eq!(count_read, count_mmap);
    assert_eq!(count_mmap, count_par_mmap);

    println!("4. Comparison");
    println!("   -----------");
    if duration_mmap < duration_read {
        let speedup = duration_read.as_secs_f64() / duration_mmap.as_secs_f64();
        println!(
            "   ✅ Memory-map search was {:.2}x faster than standard I/O.",
            speedup
        );
    } else {
        println!("   ⚠️ Memory-map search was not faster.");
    }
    
    if duration_par_mmap < duration_mmap {
        let speedup = duration_mmap.as_secs_f64() / duration_par_mmap.as_secs_f64();
        println!(
            "   ✅ Parallel memory-map search was {:.2}x faster than single-threaded mmap.",
            speedup
        );
    } else {
        println!("   ⚠️ Parallel mmap search was not faster than single-threaded mmap.");
    }

    println!("\n(Note: Performance can vary based on OS caching and file size.)");

    println!("\n=== Demo Complete! ===");
    Ok(())
}