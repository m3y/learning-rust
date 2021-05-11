use std::fmt;

fn main() {
    // In general, the `{}` will be automatically replaced with any
    // arguments. These will be stringified.
    // 一般的に `{} `はどんな引数であろうと自動的に置き換えられます。
    // 例えば以下は文字列に変換されます
    println!("{} days", 31);

    // Without a suffix, 31 becomes an i32. You can change what type 31 is
    // by providing a suffix. The number 31i64 for example has the type i64.
    // サフィックスで型を指定しなければ31はi32として扱われます。
    // サフィックスの指定により、31の型を自由に変換することができます。

    // There are various optional patterns this works with. Positional
    // arguments can be used.
    // 引数の位置から埋め込まれる場所を指定することができます。
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // As can named arguments.
    // 名前での指定も可能です。
    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );

    // Special formatting can be specified after a `:`.
    // `:` のあとにフォーマット型を指定することによる特殊なフォーマットも可能です.
    println!(
        "{} of {:b} people know binary, the other half doesn't",
        1, 2
    );

    // You can right-align text with a specified width. This will output
    // "     1". 5 white spaces and a "1".
    // 指定した幅の中に、右寄せで文字列を挿入することができます。
    // 以下の例では"     1". というように、５つの半角空白のあとに"1"が入ります.
    println!("{number:>width$}", number = 1, width = 6);

    // You can pad numbers with extra zeroes. This will output "000001".
    // 空白の代わりに0を使うこともできます. このアウトプットは "000001" になります.
    println!("{number:>0width$}", number = 1, width = 6);

    // Rust even checks to make sure the correct number of arguments are
    // used.
    // 引数の数が正しいかのチェックも行ってくれます。
    println!("My name is {0}, {1} {0}", "Bond", "James");
    // FIXME ^ Add the missing argument: "James"

    // Create a structure named `Structure` which contains an `i32`.
    // `i32`保持する `Structure` という名の構造体を定義します.
    #[allow(dead_code)]
    struct Structure(i32);

    impl fmt::Display for Structure {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    // However, custom types such as this structure require more complicated
    // handling. This will not work.
    // このようにカスタム型を用いる場合、少々扱いが複雑になります。
    // 以下は動作しません。
    println!("This struct `{}` won't print...", Structure(3));
    // FIXME ^ Comment out this line.
}
