use gittype::extractor::{CodeExtractor, ExtractionOptions, Language};
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn test_kotlin_class_extraction() {
    let kotlin_code = r#"
class MainActivity : AppCompatActivity() {
    companion object {
        const val TAG = "MainActivity"
        
        fun staticMethod(): String {
            return "Hello from static method"
        }
    }
    
    private val name: String = "GitType"
    
    fun greetUser(username: String): String {
        return "Hello, $username! Welcome to $name."
    }
}

data class User(
    val id: Long,
    val name: String,
    val email: String
) {
    fun getDisplayName(): String = "$name ($email)"
}

object DatabaseHelper {
    fun connect(): Connection {
        return DriverManager.getConnection("jdbc:sqlite:app.db")
    }
}
"#;

    let mut file = NamedTempFile::new().unwrap();
    file.write_all(kotlin_code.as_bytes()).unwrap();
    file.flush().unwrap();

    let mut extractor = CodeExtractor::new().unwrap();
    let options = ExtractionOptions::default();
    let chunks = extractor
        .extract_from_file(file.path(), Language::Kotlin, &options)
        .unwrap();

    let class_chunks: Vec<_> = chunks
        .iter()
        .filter(|chunk| chunk.name.contains("MainActivity") || chunk.name.contains("User"))
        .collect();
    assert!(
        !class_chunks.is_empty(),
        "Should extract classes MainActivity and User"
    );

    let function_chunks: Vec<_> = chunks
        .iter()
        .filter(|chunk| {
            chunk.name.contains("greetUser")
                || chunk.name.contains("getDisplayName")
                || chunk.name.contains("staticMethod")
                || chunk.name.contains("connect")
        })
        .collect();
    assert!(
        !function_chunks.is_empty(),
        "Should extract functions from classes and objects"
    );

    let object_chunks: Vec<_> = chunks
        .iter()
        .filter(|chunk| chunk.name.contains("DatabaseHelper"))
        .collect();
    assert!(
        !object_chunks.is_empty(),
        "Should extract object declarations"
    );
}
