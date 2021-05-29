# rust-parallel-benchmark

## 使い方
* Rustインストール  
```
curl https://sh.rustup.rs -sSf | sh
source $HOME/.cargo/env
```


* テスト、ベンチマーク用の準備  
```rustup install nightly```

### クローンしたら  
* ビルド  
```cargo +nightly build```

* ビルドと実行  
```cargo +nightly run --bin main```

* テスト  
```cargo +nightly test```

* ベンチマーク  
```cargo +nightly run --bin bench```  
~~回す回数が制御できずめちゃくちゃ遅いので要修正~~  
修正済み

### 並列化するには
* クイックソートの問題なら quicksort/src/quicksort.rs を編集する

### 計測回数を変えるには
* quicksort/src/bench.rs 38行目の数字を変える