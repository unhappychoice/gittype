use crate::typing_core_test_with_parser;

typing_core_test_with_parser!(
    php_function_with_comment,
    "php",
    r#"<?php
function add($a, $b) {
    return $a + $b; // Return sum
}"#
);

typing_core_test_with_parser!(
    php_class_with_comments,
    "php",
    r#"<?php
class Calculator {
    private $result = 0; // Store result
    
    public function add($value) { // Add method
        $this->result += $value;
        return $this;
    }
    
    public function getResult() {
        return $this->result; # Return current result
    }
}"#
);

typing_core_test_with_parser!(
    php_function_with_block_comment,
    "php",
    r#"<?php
function factorial($n) {
    /* Calculate factorial
       using recursion */
    if ($n <= 1) {
        return 1;
    }
    return $n * factorial($n - 1);
}"#
);

typing_core_test_with_parser!(
    php_array_with_comments,
    "php",
    r#"<?php
$config = [
    'host' => 'localhost', // Database host
    'port' => 3306,        // Database port
    'name' => 'myapp',     // Database name
    'user' => 'root'       # Database user
];"#
);

typing_core_test_with_parser!(
    php_namespace_with_comments,
    "php",
    r#"<?php
namespace App\Models;

use Illuminate\Database\Eloquent\Model;

class User extends Model {
    // Table name
    protected $table = 'users';
    
    // Fillable fields
    protected $fillable = [
        'name',
        'email',
        'password'
    ];
}"#
);
