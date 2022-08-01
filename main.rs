fn main() {
    let a_in_main = String::from("hogehoge");

    // スコープを切って、中でインスタンス生成を行う。
    // 従来のコードブロックによる区切りよりも明確になる。
    // ただし完全に隔離できるなら、いっそ関数として
    // 定義したほうがいい説もある。
    // 関数定義説へのカウンターは、インターフェースによる
    // 複雑さ増大。ぺらぺらの関数定義よりはこっちのがいい。
    let c_in_main_by_subscope = {
        let b_in_subscope = String::from("fugafuga");
        b_in_subscope // <- 式。値が返る。
    }; // <- c_in_main_by_subscope = {...} は文。

    // 波括弧はスタックに積まれて剥かれていく感じっぽい。
    // 波括弧で囲ったものは、式として評価されるのはたぶん確実
    let d_might_work = {{{{{{
        String::from("piyopiyo")
    }}}}}};

    println!("a = {}", a_in_main);
    println!("c = {}", c_in_main_by_subscope);
    println!("d = {}", d_might_work);

    // 波括弧で囲ったものが式として評価されることの検証
    let e = String::from("hogefuga");
    println!("e = {}", {e}); // <- ちゃんと剥かれてる。
}