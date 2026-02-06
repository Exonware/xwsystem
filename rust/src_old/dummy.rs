// #exonware/xwsystem/rust/src_old/dummy.rs
//! Dummy complicated functions for testing Python bindings
//!
//! Company: eXonware.com
//! Author: Eng. Muhammad AlShehri
//! Email: connect@exonware.com
//! Version: 0.1.0.1

use serde::{Deserialize, Serialize};

/// Result structure for dummy_complicated function
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DummyResult {
    pub output1: i64,
    pub output2: String,
}

/// A complicated dummy function that takes 5 inputs plus variable args
/// and returns 2 outputs
///
/// # Arguments
/// * `input1` - First integer input
/// * `input2` - Second integer input  
/// * `input3` - Third integer input
/// * `input4` - String input
/// * `input5` - Boolean input
/// * `args` - Variable number of additional integer arguments
///
/// # Returns
/// A tuple of (output1: i64, output2: String)
pub fn dummy_complicated(
    input1: i32,
    input2: i32,
    input3: i32,
    input4: String,
    input5: bool,
    args: Vec<i32>,
) -> DummyResult {
    // Complicated computation: sum all integers, multiply by factors
    let mut sum = input1 as i64 + input2 as i64 + input3 as i64;
    
    // Add all args
    for arg in &args {
        sum += *arg as i64;
    }
    
    // Apply boolean multiplier
    let multiplier = if input5 { 2 } else { 1 };
    sum *= multiplier;
    
    // Square the result for complexity
    sum = sum * sum;
    
    // Build complex output string
    let mut output2 = format!("input1={},input2={},input3={}", input1, input2, input3);
    output2.push_str(&format!(",input4={}", input4));
    output2.push_str(&format!(",input5={}", input5));
    output2.push_str(&format!(",args_count={}", args.len()));
    output2.push_str(&format!(",args_sum={}", args.iter().sum::<i32>()));
    output2.push_str(&format!(",final_sum={}", sum));
    
    DummyResult {
        output1: sum,
        output2,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dummy_complicated() {
        let result = dummy_complicated(
            10,
            20,
            30,
            "test".to_string(),
            true,
            vec![5, 15, 25],
        );
        
        // (10 + 20 + 30 + 5 + 15 + 25) * 2 = 210, then squared = 44100
        assert_eq!(result.output1, 44100);
        assert!(result.output2.contains("input1=10"));
        assert!(result.output2.contains("args_count=3"));
    }
}

