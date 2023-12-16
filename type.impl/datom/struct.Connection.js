(function() {var type_impls = {
"datom":[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Connection%3CS%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/datom/types/connection.rs.html#68-160\">source</a><a href=\"#impl-Connection%3CS%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;S: <a class=\"trait\" href=\"datom/storage/trait.Storage.html\" title=\"trait datom::storage::Storage\">Storage</a>&gt; <a class=\"struct\" href=\"datom/struct.Connection.html\" title=\"struct datom::Connection\">Connection</a>&lt;S&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.new\" class=\"method\"><a class=\"src rightside\" href=\"src/datom/types/connection.rs.html#70-75\">source</a><h4 class=\"code-header\">pub fn <a href=\"datom/struct.Connection.html#tymethod.new\" class=\"fn\">new</a>(storage: S) -&gt; Self</h4></section></summary><div class=\"docblock\"><p>Create a new connection from a storage backend</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.latest_t\" class=\"method\"><a class=\"src rightside\" href=\"src/datom/types/connection.rs.html#78-85\">source</a><h4 class=\"code-header\">pub fn <a href=\"datom/struct.Connection.html#tymethod.latest_t\" class=\"fn\">latest_t</a>(&amp;self) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u64.html\">u64</a>, <a class=\"enum\" href=\"datom/enum.ConnectionError.html\" title=\"enum datom::ConnectionError\">ConnectionError</a>&gt;</h4></section></summary><div class=\"docblock\"><p>Fetch the t-value for the latest transaction</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.as_of\" class=\"method\"><a class=\"src rightside\" href=\"src/datom/types/connection.rs.html#88-93\">source</a><h4 class=\"code-header\">pub const fn <a href=\"datom/struct.Connection.html#tymethod.as_of\" class=\"fn\">as_of</a>(&amp;self, t: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u64.html\">u64</a>) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"struct\" href=\"datom/struct.Database.html\" title=\"struct datom::Database\">Database</a>&lt;'_, S&gt;, <a class=\"enum\" href=\"datom/enum.ConnectionError.html\" title=\"enum datom::ConnectionError\">ConnectionError</a>&gt;</h4></section></summary><div class=\"docblock\"><p>Fetch the t-value for the latest transaction</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.db\" class=\"method\"><a class=\"src rightside\" href=\"src/datom/types/connection.rs.html#97-99\">source</a><h4 class=\"code-header\">pub fn <a href=\"datom/struct.Connection.html#tymethod.db\" class=\"fn\">db</a>(&amp;self) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"struct\" href=\"datom/struct.Database.html\" title=\"struct datom::Database\">Database</a>&lt;'_, S&gt;, <a class=\"enum\" href=\"datom/enum.ConnectionError.html\" title=\"enum datom::ConnectionError\">ConnectionError</a>&gt;</h4></section></summary><div class=\"docblock\"><p>Get a <a href=\"datom/struct.Database.html\" title=\"struct datom::Database\">database</a> for the current\npoint in time</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.transact_tx\" class=\"method\"><a class=\"src rightside\" href=\"src/datom/types/connection.rs.html#102-151\">source</a><h4 class=\"code-header\">pub fn <a href=\"datom/struct.Connection.html#tymethod.transact_tx\" class=\"fn\">transact_tx</a>(\n    &amp;self,\n    tx: <a class=\"struct\" href=\"datom/struct.Transaction.html\" title=\"struct datom::Transaction\">Transaction</a>\n) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"struct\" href=\"datom/struct.TransactionResult.html\" title=\"struct datom::TransactionResult\">TransactionResult</a>&lt;'_, S&gt;, <a class=\"enum\" href=\"datom/enum.TransactionError.html\" title=\"enum datom::TransactionError\">TransactionError</a>&gt;</h4></section></summary><div class=\"docblock\"><p>Run a transaction on the database</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.transact\" class=\"method\"><a class=\"src rightside\" href=\"src/datom/types/connection.rs.html#154-159\">source</a><h4 class=\"code-header\">pub fn <a href=\"datom/struct.Connection.html#tymethod.transact\" class=\"fn\">transact</a>&lt;T: <a class=\"trait\" href=\"datom/trait.Transactable.html\" title=\"trait datom::Transactable\">Transactable</a>&gt;(\n    &amp;self,\n    txable: T\n) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"struct\" href=\"datom/struct.TransactionResult.html\" title=\"struct datom::TransactionResult\">TransactionResult</a>&lt;'_, S&gt;, <a class=\"enum\" href=\"datom/enum.TransactionError.html\" title=\"enum datom::TransactionError\">TransactionError</a>&gt;</h4></section></summary><div class=\"docblock\"><p>Transact a transactable on the database</p>\n</div></details></div></details>",0,"datom::types::connection::DynamicConnection"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-PartialEq-for-Connection%3CS%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/datom/types/connection.rs.html#26-55\">source</a><a href=\"#impl-PartialEq-for-Connection%3CS%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;S: <a class=\"trait\" href=\"datom/storage/trait.Storage.html\" title=\"trait datom::storage::Storage\">Storage</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html\" title=\"trait core::cmp::PartialEq\">PartialEq</a> for <a class=\"struct\" href=\"datom/struct.Connection.html\" title=\"struct datom::Connection\">Connection</a>&lt;S&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.eq\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/datom/types/connection.rs.html#52-54\">source</a><a href=\"#method.eq\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq\" class=\"fn\">eq</a>(&amp;self, other: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;Self</a>) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\">bool</a></h4></section></summary><div class=\"docblock\">\n<div class=\"example-wrap\"><pre class=\"rust rust-example-rendered\"><code><span class=\"kw\">use </span>datom::{backends::SledStorage, Connection};\n<span class=\"kw\">let </span>storage1 = SledStorage::connect_temp()<span class=\"question-mark\">?</span>;\n<span class=\"kw\">let </span>storage2 = SledStorage::connect_temp()<span class=\"question-mark\">?</span>;\n<span class=\"kw\">let </span>conn1 = Connection::new(storage1);\n<span class=\"kw\">let </span>conn2 = Connection::new(storage2);\n<span class=\"kw\">let </span>conn1r = <span class=\"kw-2\">&amp;</span>conn1;\n<span class=\"kw\">let </span>conn2r = <span class=\"kw-2\">&amp;</span>conn2;\n\n<span class=\"macro\">assert_eq!</span>(<span class=\"kw-2\">&amp;</span>conn1, <span class=\"kw-2\">&amp;</span>conn1);\n<span class=\"macro\">assert_eq!</span>(<span class=\"kw-2\">&amp;</span>conn1, conn1r);\n<span class=\"macro\">assert_eq!</span>(conn1r, <span class=\"kw-2\">&amp;</span>conn1);\n<span class=\"macro\">assert_eq!</span>(conn1r, conn1r);\n\n<span class=\"macro\">assert_ne!</span>(<span class=\"kw-2\">&amp;</span>conn1, <span class=\"kw-2\">&amp;</span>conn2);\n<span class=\"macro\">assert_ne!</span>(<span class=\"kw-2\">&amp;</span>conn1, conn2r);\n<span class=\"macro\">assert_ne!</span>(conn1r, <span class=\"kw-2\">&amp;</span>conn2);\n<span class=\"macro\">assert_ne!</span>(conn1r, conn2r);\n<span class=\"macro\">assert_ne!</span>(<span class=\"kw-2\">&amp;</span>conn2, <span class=\"kw-2\">&amp;</span>conn1);\n<span class=\"macro\">assert_ne!</span>(<span class=\"kw-2\">&amp;</span>conn2, conn1r);\n<span class=\"macro\">assert_ne!</span>(conn2r, <span class=\"kw-2\">&amp;</span>conn1);\n<span class=\"macro\">assert_ne!</span>(conn2r, conn1r);\n</code></pre></div>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.ne\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#242\">source</a></span><a href=\"#method.ne\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne\" class=\"fn\">ne</a>(&amp;self, other: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;Rhs</a>) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\">bool</a></h4></section></summary><div class='docblock'>This method tests for <code>!=</code>. The default implementation is almost always\nsufficient, and should not be overridden without very good reason.</div></details></div></details>","PartialEq","datom::types::connection::DynamicConnection"],["<section id=\"impl-Eq-for-Connection%3CS%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/datom/types/connection.rs.html#57\">source</a><a href=\"#impl-Eq-for-Connection%3CS%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;S: <a class=\"trait\" href=\"datom/storage/trait.Storage.html\" title=\"trait datom::storage::Storage\">Storage</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html\" title=\"trait core::cmp::Eq\">Eq</a> for <a class=\"struct\" href=\"datom/struct.Connection.html\" title=\"struct datom::Connection\">Connection</a>&lt;S&gt;</h3></section>","Eq","datom::types::connection::DynamicConnection"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Debug-for-Connection%3CS%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/datom/types/connection.rs.html#162-208\">source</a><a href=\"#impl-Debug-for-Connection%3CS%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;S: <a class=\"trait\" href=\"datom/storage/trait.Storage.html\" title=\"trait datom::storage::Storage\">Storage</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"struct\" href=\"datom/struct.Connection.html\" title=\"struct datom::Connection\">Connection</a>&lt;S&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/datom/types/connection.rs.html#171-207\">source</a><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt\" class=\"fn\">fmt</a>(&amp;self, f: &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html\" title=\"struct core::fmt::Formatter\">Formatter</a>&lt;'_&gt;) -&gt; <a class=\"type\" href=\"https://doc.rust-lang.org/nightly/core/fmt/type.Result.html\" title=\"type core::fmt::Result\">Result</a></h4></section></summary><div class=\"docblock\">\n<div class=\"example-wrap\"><pre class=\"rust rust-example-rendered\"><code><span class=\"kw\">use </span>datom::{Connection, backends::SledStorage};\n\n<span class=\"kw\">let </span>storage = SledStorage::connect_temp()<span class=\"question-mark\">?</span>;\n<span class=\"kw\">let </span>conn = Connection::new(storage);\n<span class=\"macro\">println!</span>(<span class=\"string\">\"{:#?}\"</span>, conn);</code></pre></div>\n</div></details></div></details>","Debug","datom::types::connection::DynamicConnection"]]
};if (window.register_type_impls) {window.register_type_impls(type_impls);} else {window.pending_type_impls = type_impls;}})()