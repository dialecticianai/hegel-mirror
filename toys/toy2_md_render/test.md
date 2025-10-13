# Test Markdown Document

![Hegel Logo](../../logo.png)

This is a **test** document with _various_ Markdown features.

## Features

- Lists
- **Bold** and *italic*
- `inline code`

### Code Blocks

**Rust with pattern matching:**
```rust
fn process_result(result: Result<i32, String>) -> i32 {
    match result {
        Ok(value) => {
            println!("Success: {}", value);
            value * 2
        }
        Err(error) => {
            eprintln!("Error: {}", error);
            0
        }
    }
}
```

**Python with list comprehension:**
```python
def fibonacci(n):
    """Generate fibonacci sequence up to n terms"""
    if n <= 0:
        return []
    elif n == 1:
        return [0]

    fib = [0, 1]
    return fib + [fib[i-1] + fib[i-2] for i in range(2, n)]

# Example usage
result = fibonacci(10)
print(f"First 10 fibonacci numbers: {result}")
```

**JavaScript with async/await:**
```javascript
async function fetchUserData(userId) {
    try {
        const response = await fetch(`/api/users/${userId}`);
        const data = await response.json();

        return {
            ...data,
            timestamp: new Date().toISOString()
        };
    } catch (error) {
        console.error('Failed to fetch user:', error);
        throw error;
    }
}
```

**Go with goroutines:**
```go
package main

import (
    "fmt"
    "sync"
    "time"
)

func worker(id int, jobs <-chan int, results chan<- int, wg *sync.WaitGroup) {
    defer wg.Done()
    for job := range jobs {
        fmt.Printf("Worker %d processing job %d\n", id, job)
        time.Sleep(time.Second)
        results <- job * 2
    }
}
```

### Links

[Mirror Project](https://github.com/dialecticianai/mirror)

### Quotes

> This is a blockquote.
> It can span multiple lines.

## Conclusion

Testing CommonMark rendering in egui.
