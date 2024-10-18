use anyhow::{Result, anyhow};

pub enum Operand {
    Int(i64),
    Float(f64),
}

impl Operand {
    pub fn add(self, other: Operand) -> String {
        match (self, other) {
            (Operand::Int(op1), Operand::Int(op2)) => (op1 + op2).to_string(),
            (Operand::Float(op1), Operand::Float(op2)) => (op1 + op2).to_string(),
            (Operand::Int(op1), Operand::Float(op2)) => (op1 as f64 + op2).to_string(),
            (Operand::Float(op1), Operand::Int(op2)) => (op1 + op2 as f64).to_string(),
        }
    }

    pub fn subtract(self, other: Operand) -> String {
        match (self, other) {
            (Operand::Int(op1), Operand::Int(op2)) => (op1 - op2).to_string(),
            (Operand::Float(op1), Operand::Float(op2)) => (op1 - op2).to_string(),
            (Operand::Int(op1), Operand::Float(op2)) => (op1 as f64 - op2).to_string(),
            (Operand::Float(op1), Operand::Int(op2)) => (op1 - op2 as f64).to_string(),
        }
    }

    pub fn multiply(self, other: Operand) -> String {
        match (self, other) {
            (Operand::Int(op1), Operand::Int(op2)) => (op1 * op2).to_string(),
            (Operand::Float(op1), Operand::Float(op2)) => (op1 * op2).to_string(),
            (Operand::Int(op1), Operand::Float(op2)) => (op1 as f64 * op2).to_string(),
            (Operand::Float(op1), Operand::Int(op2)) => (op1 * op2 as f64).to_string(),
        }
    }

    pub fn divide(self, other: Operand) -> Result<String> {
        match (self, other) {
            (Operand::Int(op1), Operand::Int(op2)) => {
                if op2 == 0 {
                    Err(anyhow!("NaN"))
                } else {
                    Ok((op1 as f64 / op2 as f64).to_string())
                }
            }
            (Operand::Float(op1), Operand::Float(op2)) => {
                if op2 == 0.0 {
                    Err(anyhow!("NaN"))
                } else {
                    Ok((op1 / op2).to_string())
                }
            }
            (Operand::Int(op1), Operand::Float(op2)) => {
                if op2 == 0.0 {
                    Err(anyhow!("NaN"))
                } else {
                    Ok((op1 as f64 / op2).to_string())
                }
            }
            (Operand::Float(op1), Operand::Int(op2)) => {
                if op2 == 0 {
                    Err(anyhow!("NaN"))
                } else {
                    Ok((op1 / op2 as f64).to_string())
                }
            }
        }
    }
}