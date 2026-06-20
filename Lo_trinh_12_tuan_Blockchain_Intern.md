# LỘ TRÌNH 12 TUẦN: BLOCKCHAIN DEVELOPER INTERN (RUST / SOLANA)
### Chương trình đào tạo thực chiến — dành cho người đã có nền Rust + Axum + PostgreSQL + Vue.js


## 0. NGUYÊN TẮC THIẾT KẾ & ƯU TIÊN


| Ưu tiên | Mảng kiến thức | Lý do |
|---|---|---|
| 1 (cao nhất) | Solana account model, Anchor, SPL Token, PDA, CPI | Đây là thứ JD Intern Blockchain (Rust) chắc chắn hỏi & test code |
| 2 | Rust nâng cao áp dụng cho on-chain (Borsh, error handling, security) | Phân biệt bạn với người chỉ biết Rust "web" |
| 3 | Cryptography cơ bản (hash, ECC, ký số) | Câu hỏi nền tảng, dễ bị hỏi xoáy |
| 4 | Bitcoin/Ethereum architecture | Câu hỏi so sánh kiến trúc — không cần code sâu, cần **hiểu & nói được** |
| 5 | DeFi/NFT primitives | Thể hiện bạn hiểu use-case thực tế, không chỉ biết cú pháp |

**Cấu trúc mỗi tuần:** 6 ngày học chủ động (4-6h/ngày) + 1 ngày checkpoint/ôn tập/đẩy code lên GitHub.
**Nguyên tắc:** Mỗi tuần phải có ít nhất 1 dòng code thật chạy được — không học lý thuyết suông.


## GIAI ĐOẠN 1 — TUẦN 1-2: BLOCKCHAIN FUNDAMENTALS + CRYPTOGRAPHY

**Mục tiêu giai đoạn:** Hiểu vì sao blockchain hoạt động được (hash, Merkle tree, consensus, chữ ký số) và tự code lại các nguyên lý đó bằng Rust — đây là nền móng để hiểu sâu Solana/Ethereum sau này, và là nhóm câu hỏi phỏng vấn "warm-up" mà gần như intern nào cũng bị hỏi.

### Tuần 1 — Nguyên lý cốt lõi Blockchain

| Ngày | Nội dung | Tài liệu chính thức | Thực hành |
|---|---|---|---|
| 1 | Decentralization, distributed ledger, block structure, vì sao cần blockchain | [Bitcoin Whitepaper – Satoshi Nakamoto](https://bitcoin.org/bitcoin.pdf) (đọc mục 1-3) | Viết tóm tắt 1 trang bằng tiếng Việt: block, chain, ledger là gì. Vẽ sơ đồ block liên kết nhau. |
| 2 | Cryptographic hash function: SHA-256, Keccak256, tính chất (deterministic, avalanche effect, collision resistance) | [RustCrypto/hashes repo](https://github.com/RustCrypto/hashes) | Dùng crate `sha2` viết hàm `hash_block(data: &[u8]) -> [u8;32]`. Test avalanche effect: đổi 1 bit input, so sánh output. |
  | 3 | Merkle Tree: cấu trúc, Merkle root, Merkle proof | [Bitcoin Developer Guide – Merkle Trees](https://developer.bitcoin.org/devguide/block_chain.html#merkle-trees) | Code `MerkleTree` struct bằng Rust: build tree từ list transaction, sinh proof, viết hàm `verify_proof()`. |
| 4 | Public-key cryptography: asymmetric encryption, digital signature là gì, vì sao blockchain cần nó | [Cloudflare – ECC blog series](https://blog.cloudflare.com/a-relatively-easy-to-understand-primer-on-elliptic-curve-cryptography/) | Dùng crate `ed25519-dalek`: generate keypair, sign message, verify signature. |
| 5 | Consensus: Proof of Work, Proof of Stake, BFT-style (giới thiệu sơ Tower BFT của Solana) | [Ethereum.org – Consensus Mechanisms](https://ethereum.org/en/developers/docs/consensus-mechanisms/) | Code PoW miner đơn giản: tìm nonce sao cho `hash(block) < target`. |
| 6 | **Project ngày**: Ghép tất cả lại — mini blockchain | — | Xây `rust-mini-blockchain`: Block { index, prev_hash, merkle_root, nonce, timestamp, transactions }, có sign transaction + verify + PoW. |
| 7 | **Checkpoint** | — | Tự trả lời: "Giải thích cho người không biết kỹ thuật: blockchain là gì trong 3 câu?" + đẩy code lên GitHub repo `rust-mini-blockchain` |

**GitHub project tuần 1:** `rust-mini-blockchain` — block, hash chain, Merkle tree, PoW, ký số.

### Tuần 2 — Cryptography chuyên sâu (áp dụng thực tế cho Bitcoin/Ethereum/Solana)

| Ngày | Nội dung | Tài liệu chính thức | Thực hành |
|---|---|---|---|
| 1 | Elliptic Curve Cryptography — trực giác toán học (không cần chứng minh), curve secp256k1 | [secp256k1 spec – SEC2](https://www.secg.org/sec2-v2.pdf) (đọc giới thiệu) | Vẽ sơ đồ point addition trên đường cong (chỉ cần hiểu trực giác, không cần tính tay). |
| 2 | secp256k1 (Bitcoin/Ethereum) vs ed25519 (Solana): khác biệt, lý do Solana chọn ed25519 | [Solana Docs – Keys & Wallets](https://solana.com/docs/intro/wallets) | Tạo keypair Solana bằng `solana-keygen new`, dùng `solana-keygen verify` kiểm tra. |
| 3 | Address derivation: từ public key ra address (Bitcoin: hash160, Ethereum: Keccak256 + lấy 20 byte cuối, Solana: base58(pubkey) trực tiếp) | [Ethereum.org – Accounts](https://ethereum.org/en/developers/docs/accounts/) | Viết Rust function tự tính address Ethereum từ public key (dùng `sha3`/`tiny-keccak`), so sánh với MetaMask thật. |
| 4 | BIP32/BIP39/BIP44 — mnemonic, seed, HD wallet (kiến thức nền tảng mọi chain dùng) | [BIP-39 spec](https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki) | Dùng crate `bip39` sinh mnemonic 12 từ → seed → derive keypair Solana từ seed đó. |
| 5 | Transaction signing flow tổng quát: build message → hash → sign → broadcast → verify | [Solana Docs – Transactions](https://solana.com/docs/core/transactions) | Viết Rust struct `Transaction { payload, signature }`, hàm `sign()` và `verify()` độc lập (không phụ thuộc chain nào). |
| 6 | **Project ngày**: Wallet CLI tool | — | Xây `rust-wallet-cli`: generate keypair, lưu vào file (mã hoá đơn giản), sign message, verify signature, hiển thị address. Đây là **hạt giống Portfolio Project**. |
| 7 | **Checkpoint** | — | Quiz tự kiểm: "Tại sao Solana nhanh hơn nếu dùng ed25519 thay vì ECDSA secp256k1?" + push `rust-wallet-cli` lên GitHub |

**GitHub project tuần 2:** `rust-wallet-cli` — CLI tạo ví, ký, verify (nền cho phần ký giao dịch Solana sau).

---

## GIAI ĐOẠN 2 — TUẦN 3-4: BITCOIN & ETHEREUM ARCHITECTURE

**Mục tiêu giai đoạn:** Hiểu đủ sâu 2 kiến trúc blockchain "kinh điển" để (a) trả lời tốt câu hỏi so sánh kiến trúc trong phỏng vấn, (b) hiểu rõ Solana khác biệt ở điểm nào và tại sao — đây là câu hỏi rất hay gặp ("Tại sao chọn Solana, so với Ethereum?").

### Tuần 3 — Bitcoin Architecture

| Ngày | Nội dung | Tài liệu chính thức | Thực hành |
|---|---|---|---|
| 1 | UTXO model (Unspent Transaction Output) vs Account model | [Bitcoin Developer Guide – Transactions](https://developer.bitcoin.org/devguide/transactions.html) | Vẽ sơ đồ: 1 transaction tiêu 2 UTXO input, tạo 2 UTXO output (1 cho người nhận, 1 change). |
| 2 | Cấu trúc transaction Bitcoin: scriptSig, scriptPubKey, P2PKH | [Bitcoin Developer Reference – Transactions](https://developer.bitcoin.org/reference/transactions.html) | Dùng crate `bitcoin` (rust-bitcoin) để decode 1 raw transaction hex lấy từ [Blockstream Explorer](https://blockstream.info) thành struct. |
| 3 | Block header, mining difficulty, Merkle root trong block | [Bitcoin Developer Guide – Block Chain](https://developer.bitcoin.org/devguide/block_chain.html) | Lấy 1 block thật qua Blockstream API (`GET /block/:hash`), parse field bằng Rust, tự tính lại Merkle root từ list tx, so khớp. |
| 4 | Bitcoin Script: opcode cơ bản, multisig (P2SH) | [Bitcoin Script wiki](https://en.bitcoin.it/wiki/Script) | Viết ví dụ script `OP_2 OP_3 OP_CHECKMULTISIG` trên giấy/markdown, giải thích flow chạy. |
| 5 | Lightning Network — tổng quan (đủ để nói trong phỏng vấn, không cần code) | [Lightning Network whitepaper](https://lightning.network/lightning-network-paper.pdf) (đọc abstract + mục 1-2) | Viết tóm tắt 5 câu: payment channel giải quyết vấn đề gì. |
| 6 | **Project ngày**: Bitcoin tx parser | — | Xây `bitcoin-tx-parser`: CLI nhận raw tx hex, in ra inputs/outputs/script/fee dạng JSON đẹp. |
| 7 | **Checkpoint** | — | Tự hỏi: "UTXO model có ưu/nhược gì so với account model?" + push `bitcoin-tx-parser` |

**GitHub project tuần 3:** `bitcoin-tx-parser`.

### Tuần 4 — Ethereum Architecture (và bắt đầu so sánh với Solana)

| Ngày | Nội dung | Tài liệu chính thức | Thực hành |
|---|---|---|---|
| 1 | Account-based model, state, nonce, balance, World State (Merkle Patricia Trie) | [Ethereum.org – Accounts](https://ethereum.org/en/developers/docs/accounts/) | So sánh bằng bảng: Bitcoin UTXO vs Ethereum Account vs Solana Account (sẽ hoàn thiện ở ngày 5). |
| 2 | EVM: bytecode, opcode, gas, execution model | [Ethereum.org – EVM](https://ethereum.org/en/developers/docs/evm/) | Deploy 1 smart contract Solidity siêu đơn giản (counter) qua [Remix IDE](https://remix.ethereum.org) lên testnet (Sepolia) — chỉ để cảm nhận flow, không cần thành thạo Solidity. |
| 3 | Smart contract standards: ERC-20, ERC-721 (đọc hiểu interface, không cần viết) | [EIP-20](https://eips.ethereum.org/EIPS/eip-20), [EIP-721](https://eips.ethereum.org/EIPS/eip-721) | Đọc interface ERC-20 (`transfer`, `approve`, `balanceOf`), ghi chú để so sánh với SPL Token tuần 9. |
| 4 | Gas mechanics, EIP-1559 (base fee + priority fee) | [Ethereum.org – Gas](https://ethereum.org/en/developers/docs/gas/) | Tự tính ví dụ: 1 transaction tốn 21000 gas, gasPrice 30 gwei → phí thực tế bằng bao nhiêu ETH. |
| 5 | **So sánh kiến trúc Bitcoin vs Ethereum vs Solana** (bảng tổng hợp) | [Solana Docs – Comparing Solana](https://solana.com/docs) | Viết file `comparison.md`: account model, consensus, ngôn ngữ, finality time, TPS, fee model — đây là tài liệu ôn phỏng vấn cực quan trọng. |
| 6 | **Project ngày**: Rust client gọi Ethereum testnet | — | Dùng crate `ethers-rs` (hoặc `alloy`) viết Rust script: connect Sepolia RPC, query balance 1 address, lấy latest block number. |
| 7 | **Checkpoint** | — | Quiz: "Vì sao Solana đạt TPS cao hơn Ethereum L1? Nêu 3 lý do kỹ thuật." + hoàn thiện `comparison.md` |

**GitHub project tuần 4:** script Rust query Ethereum (gộp vào repo `rust-wallet-cli` như 1 module mới, hoặc repo riêng `eth-rust-client`).

---

## GIAI ĐOẠN 3 — TUẦN 5-6: RUST NÂNG CAO CHO BLOCKCHAIN + NHẬP MÔN SOLANA

**Mục tiêu giai đoạn:** Lấp đầy khoảng cách giữa "Rust biết viết Axum backend" và "Rust viết on-chain program" — đó là serialization (Borsh), kiểm soát memory/layout, error handling theo chuẩn on-chain, và làm quen Solana runtime/account model **trước khi** chạm vào Anchor (để hiểu Anchor đang "che" cái gì).

### Tuần 5 — Rust nâng cao cho on-chain

| Ngày | Nội dung | Tài liệu chính thức | Thực hành |
|---|---|---|---|
| 1 | Borsh serialization — vì sao Solana dùng Borsh thay vì serde_json/bincode | [Borsh specification](https://borsh.io) | Viết struct `Account { owner: Pubkey, balance: u64 }`, derive `BorshSerialize`/`BorshDeserialize`, serialize/deserialize, in ra bytes. |
| 2 | Generic & trait nâng cao, static dispatch vs dynamic dispatch (ảnh hưởng compute unit) | [Rust Book – Ch.10, 17](https://doc.rust-lang.org/book/ch10-00-generics.html) | Viết 2 version cùng 1 hàm: dùng generic `<T: Trait>` và dùng `dyn Trait`, so sánh binary size bằng `cargo bloat`. |
| 3 | Memory layout: `#[repr(C)]`, `#[repr(packed)]`, alignment/padding — quan trọng vì account data Solana là raw bytes | [Rust Reference – Type Layout](https://doc.rust-lang.org/reference/type-layout.html) | Dùng `std::mem::size_of`/`align_of` kiểm tra struct trước/sau khi thêm `#[repr(C)]`. |
| 4 | Macro cơ bản: `macro_rules!` — cần để đọc hiểu Anchor macro sau này (không cần viết proc-macro) | [Rust Book – Ch.19.5 Macros](https://doc.rust-lang.org/book/ch19-06-macros.html) | Viết macro đơn giản tự sinh hàm `getter`/`setter` cho struct. |
| 5 | Error handling chuẩn production: `thiserror`, custom error enum, `Result` propagation, so với cách Solana program trả lỗi (`ProgramError`) | [thiserror docs](https://docs.rs/thiserror) | Refactor lại 1 phần code Axum cũ của bạn dùng `thiserror` cho custom error đẹp hơn `Box<dyn Error>`. |
| 6 | **Project ngày**: Borsh playground + benchmark | — | Repo `borsh-playground`: serialize/deserialize struct phức tạp (nested struct, Vec, Option), viết unit test, benchmark Borsh vs serde_json bằng `criterion`. |
| 7 | **Checkpoint** | — | Tự hỏi: "Vì sao on-chain program tránh dùng `Box`, `Rc`, heap allocation tuỳ tiện?" + push `borsh-playground` |

**GitHub project tuần 5:** `borsh-playground`.

### Tuần 6 — Solana Fundamentals (Native Program, chưa dùng Anchor)

| Ngày | Nội dung | Tài liệu chính thức | Thực hành |
|---|---|---|---|
| 1 | Solana account model: mọi thứ là account, executable vs data account, rent | [Solana Docs – Accounts](https://solana.com/docs/core/accounts) | Dùng `solana account <address>` CLI xem 1 account thật trên devnet, đọc field `lamports`, `owner`, `data`. |
| 2 | Transaction & Instruction: program_id, accounts list, data (instruction data) | [Solana Docs – Transactions](https://solana.com/docs/core/transactions) | Vẽ sơ đồ 1 transaction Solana gồm nhiều instruction, mỗi instruction gọi 1 program. |
| 3 | Program Derived Address (PDA): vì sao cần, cách derive, bump seed | [Solana Docs – PDA](https://solana.com/docs/core/pda) | Viết Rust code dùng `Pubkey::find_program_address(&[seed], &program_id)`, thử với nhiều seed khác nhau. |
| 4 | Cross-Program Invocation (CPI) — tổng quan cách 1 program gọi program khác | [Solana Docs – CPI](https://solana.com/docs/core/cpi) | Đọc source code SPL Token program (phần `invoke()`), ghi chú flow. |
| 5 | Setup môi trường: `solana-test-validator`, `solana-keygen`, `solana program deploy`, Solana CLI toolchain | [Solana Docs – Installation](https://solana.com/docs/intro/installation) | Cài Solana CLI + Rust toolchain (`cargo build-sbf`), chạy local validator, airdrop SOL test. |
| 6 | **Project ngày**: Native Counter Program | — | Repo `solana-native-counter`: viết program thuần (`solana-program` crate, không Anchor) có instruction `Initialize` và `Increment`, deploy lên localnet, viết Rust client gọi nó. |
| 7 | **Checkpoint** | — | Quiz: "Giải thích flow đầy đủ từ lúc client gửi transaction đến lúc program ghi data vào account." + push `solana-native-counter` |

**GitHub project tuần 6:** `solana-native-counter` (native program — điểm cộng lớn khi phỏng vấn vì đa số candidate chỉ biết Anchor mà không hiểu "bên dưới").

---

## GIAI ĐOẠN 4 — TUẦN 7-8: ANCHOR FRAMEWORK + SMART CONTRACT DEVELOPMENT

**Mục tiêu giai đoạn:** Thành thạo Anchor — framework chuẩn công nghiệp để viết Solana program — và build được 1 program có độ phức tạp thực tế (escrow), bao gồm CPI, PDA, test, và tư duy security cơ bản.

### Tuần 7 — Anchor Framework

| Ngày | Nội dung | Tài liệu chính thức | Thực hành |
|---|---|---|---|
| 1 | Cài đặt Anchor (`avm`, `anchor-cli`), cấu trúc project, `Anchor.toml`, IDL là gì | [Anchor Docs – Installation](https://www.anchor-lang.com/docs/installation) | `anchor init my_first_program`, chạy `anchor build`, xem file IDL JSON sinh ra. |
| 2 | Macro `#[program]`, instruction handler, `Context<T>` | [Anchor Docs – Program Module](https://www.anchor-lang.com/docs) | Viết lại Counter Program (tuần 6) bằng Anchor, so sánh số dòng code với bản native. |
| 3 | Macro `#[derive(Accounts)]`: constraint `init`, `payer`, `space`, `mut`, `seeds`, `bump`, `has_one` | [Anchor Docs – Account Constraints](https://www.anchor-lang.com/docs/account-constraints) | Thêm account PDA vào Counter Program, dùng `seeds`/`bump` thay vì account thường. |
| 4 | Custom error với `#[error_code]`, sự kiện với `emit!` | [Anchor Docs – Errors](https://www.anchor-lang.com/docs/errors) | Thêm custom error (ví dụ `Overflow`, `Unauthorized`) thay cho `panic!`. |
| 5 | Testing Anchor bằng TypeScript (Mocha) — cần biết dù bạn ưu tiên Rust, vì 90% project Anchor test bằng TS | [Anchor Docs – Testing](https://www.anchor-lang.com/docs/testing/local-validator) | Viết file `tests/counter.ts` test đầy đủ happy path + 1 case lỗi (unauthorized). |
| 6 | **Project ngày**: Hoàn thiện Anchor Counter | — | Repo `anchor-counter`: Initialize, Increment, Decrement, Reset, đầy đủ test, README mô tả account diagram. |
| 7 | **Checkpoint** | — | Quiz: "Anchor `#[derive(Accounts)]` đang tự động làm những validation gì mà code native phải tự viết tay?" + push `anchor-counter` |

**GitHub project tuần 7:** `anchor-counter`.

### Tuần 8 — Smart Contract Development: Escrow Program (Project thực chiến #1)

| Ngày | Nội dung | Tài liệu chính thức | Thực hành |
|---|---|---|---|
| 1 | Thiết kế escrow program: use-case (A muốn đổi token X lấy token Y của B mà không cần trung gian), thiết kế account/state | [Anchor Book – Escrow example](https://www.anchor-lang.com/docs) + [SPL Token Docs](https://www.spl.solana.com/token) | Vẽ sơ đồ account: EscrowState, Vault (token account do PDA quản lý), Initializer, Taker. |
| 2 | Implement `initialize_escrow`: tạo vault PDA, transfer token initializer vào vault | [Anchor Docs – CPI](https://www.anchor-lang.com/docs/cross-program-invocations) | Code instruction `initialize`, dùng `anchor_spl::token::transfer` CPI sang SPL Token program. |
| 3 | Implement `exchange`: taker gửi token Y, nhận token X từ vault, đóng vault | — | Code instruction `exchange` với kiểm tra `has_one`, amount khớp. |
| 4 | Implement `cancel`: initializer rút lại token, đóng escrow | — | Code instruction `cancel`, đảm bảo chỉ initializer mới cancel được (signer check). |
| 5 | Viết test đầy đủ: happy path, taker gửi sai amount (phải fail), người ngoài cố cancel (phải fail) | [Anchor Docs – Testing](https://www.anchor-lang.com/docs/testing/local-validator) | Viết ít nhất 5 test case trong `tests/escrow.ts`. |
| 6 | Security review: integer overflow (`checked_add`), missing signer check, account ownership confusion, re-init attack | [Solana Program Security Best Practices](https://github.com/coral-xyz/sealevel-attacks) (repo ví dụ tấn công thực tế — rất nên đọc) | Tự rà lại code escrow theo checklist `sealevel-attacks`, sửa các lỗ hổng tìm thấy. |
| 7 | **Checkpoint** | — | Deploy lên devnet, demo bằng script, viết README đầy đủ. Push `anchor-escrow` — **đây là Portfolio Project #1**. |

**GitHub project tuần 8:** `anchor-escrow` ⭐ Portfolio Project #1.

---

## GIAI ĐOẠN 5 — TUẦN 9-10: TOKEN PROGRAM (SPL) + NFT PROGRAM (METAPLEX)

**Mục tiêu giai đoạn:** Thành thạo 2 chuẩn quan trọng nhất trong ecosystem Solana — SPL Token (tương đương ERC-20) và Metaplex Token Metadata (tương đương ERC-721) — vì hầu hết job thực tế (DeFi, NFT, GameFi) đều build trên 2 chuẩn này.

### Tuần 9 — SPL Token Program

| Ngày | Nội dung | Tài liệu chính thức | Thực hành |
|---|---|---|---|
| 1 | SPL Token program: Mint account, Token account, Associated Token Account (ATA) | [SPL Token Docs](https://www.spl.solana.com/token) | Đọc kỹ sơ đồ Mint → Token Account → owner, ghi chú khác biệt với Ethereum ERC-20 (Ethereum: balance nằm trong contract storage; Solana: balance nằm trong account riêng của user). |
| 2 | Thực hành CLI: tạo token, mint, transfer | [Solana Docs – spl-token CLI](https://www.solana-program.com/docs/token) | `spl-token create-token`, `create-account`, `mint`, `transfer` trên devnet — tự tay làm 1 lượt đầy đủ. |
| 3 | Implement bằng Anchor: tạo Mint, mint_to, transfer dùng crate `anchor-spl` | [Anchor SPL docs](https://docs.rs/anchor-spl) | Viết Anchor program có instruction `create_mint` và `mint_tokens`. |
| 4 | Token-2022 (Token Extensions) — tổng quan: transfer fee, metadata pointer, transfer hook | [Solana Docs – Token Extensions](https://solana.com/developers/guides/token-extensions/getting-started) | Đọc, ghi chú 3 extension hay dùng nhất trong thực tế, không cần code hết. |
| 5 | Thiết kế "Token Faucet": user request token, program mint cho họ, giới hạn theo thời gian (chống spam) | — | Thiết kế account: `FaucetConfig` (PDA), giới hạn `last_claim_timestamp` per user (PDA theo user). |
| 6 | **Project ngày**: Implement & test Token Faucet | — | Repo `spl-token-faucet`: instruction `initialize_faucet`, `claim_token` (check thời gian cooldown), test đầy đủ, deploy devnet. |
| 7 | **Checkpoint** | — | Quiz: "Vì sao Solana cần Associated Token Account thay vì chỉ cần 1 Token Account bất kỳ?" + push `spl-token-faucet` — **Portfolio Project #2 (ứng viên)**. |

**GitHub project tuần 9:** `spl-token-faucet`.

### Tuần 10 — NFT Program (Metaplex)

| Ngày | Nội dung | Tài liệu chính thức | Thực hành |
|---|---|---|---|
| 1 | NFT trên Solana: Metadata Account, Master Edition, Token Metadata program | [Metaplex Docs – Token Metadata](https://developers.metaplex.com/token-metadata) | Đọc sơ đồ: 1 NFT = 1 Mint (supply=1, decimals=0) + 1 Metadata account gắn kèm. |
| 2 | Thực hành nhanh qua Metaplex SDK/CLI để cảm nhận flow thật (mint 1 NFT lên devnet) | [Metaplex Sugar CLI](https://developers.metaplex.com/candy-machine/sugar) hoặc [Umi SDK](https://developers.metaplex.com/umi) | Mint 1 NFT test trên devnet, xem trên [Solana Explorer](https://explorer.solana.com/?cluster=devnet). |
| 3 | Implement CPI gọi Token Metadata program để tạo metadata khi mint NFT trong Anchor program | [Metaplex Token Metadata – Rust SDK](https://developers.metaplex.com/token-metadata/getting-started) | Viết instruction `mint_nft` trong Anchor: tạo mint (supply 1, decimals 0) + CPI tạo metadata (name, symbol, uri) + tạo master edition. |
| 4 | Candy Machine — tổng quan kiến trúc cho launch collection (không cần build) | [Metaplex Docs – Candy Machine](https://developers.metaplex.com/candy-machine) | Đọc, ghi chú flow: config line, guard (giá, whitelist), mint. |
| 5 | Hoàn thiện Anchor NFT Minter: thêm royalty (seller_fee_basis_points), creators array | — | Bổ sung field metadata đầy đủ chuẩn Metaplex. |
| 6 | **Project ngày**: Test & deploy | — | Repo `anchor-nft-minter`: test mint NFT, verify metadata xuất hiện đúng trên Explorer, viết script client. |
| 7 | **Checkpoint** | — | Quiz: "NFT khác Fungible Token ở điểm nào trong cấu trúc account Solana?" + push `anchor-nft-minter` — **Portfolio Project #3 (ứng viên)**. |

**GitHub project tuần 10:** `anchor-nft-minter`.

---

## GIAI ĐOẠN 6 — TUẦN 11-12: DEFI CƠ BẢN + HOÀN THIỆN PORTFOLIO + LUYỆN PHỎNG VẤN

**Mục tiêu giai đoạn:** Hiểu các DeFi primitive quan trọng nhất (AMM, staking, lending) đủ để trả lời phỏng vấn và build 1 program staking thật, sau đó dồn toàn lực hoàn thiện 3 project portfolio + luyện phỏng vấn.

### Tuần 11 — DeFi cơ bản + Staking Program

| Ngày | Nội dung | Tài liệu chính thức | Thực hành |
|---|---|---|---|
| 1 | AMM (Automated Market Maker): constant product formula `x*y=k`, liquidity pool, slippage, impermanent loss | [Uniswap V2 Whitepaper](https://uniswap.org/whitepaper.pdf) (đọc mục 2-3, khái niệm áp dụng chung mọi chain) | Tự tính ví dụ bằng tay: pool có 100 SOL & 10,000 USDC, swap 1 SOL → nhận bao nhiêu USDC theo công thức `x*y=k`. |
| 2 | Solana DeFi ecosystem: Raydium (AMM), Orca (concentrated liquidity) — tổng quan kiến trúc | [Orca Docs](https://docs.orca.so) | Đọc tổng quan, ghi chú khác biệt AMM thường vs concentrated liquidity (Whirlpool). |
| 3 | Lending/Borrowing: collateral, health factor, liquidation — tổng quan (không cần build) | [Solend / Kamino Docs](https://docs.kamino.finance) | Ghi chú flow: deposit collateral → borrow → health factor giảm dần → liquidation khi dưới ngưỡng. |
| 4 | Staking mechanism: thiết kế reward theo thời gian (reward per second/epoch) | — | Thiết kế account: `StakePool`, `UserStake` (PDA theo user) lưu `staked_amount`, `last_update_timestamp`. |
| 5 | Implement `stake` & tính reward (dùng `checked_mul`/`checked_div`, tránh overflow) | [Solana Docs – Program Security](https://solana.com/developers/courses/program-security) | Code instruction `stake`, `update_rewards` (tính reward dồn theo thời gian trôi). |
| 6 | **Project ngày**: Implement `claim_rewards`, `unstake` + test đầy đủ | — | Repo `anchor-staking-pool`: hoàn thiện 4 instruction, test các case: stake nhiều lần, claim, unstake một phần. |
| 7 | **Checkpoint** | — | Quiz: "Tại sao phải dùng `checked_add`/`checked_mul` thay vì `+`/`*` thường trong on-chain program?" + push `anchor-staking-pool`, deploy devnet. |

**GitHub project tuần 11:** `anchor-staking-pool`.

### Tuần 12 — Hoàn thiện Portfolio + Luyện Phỏng vấn

| Ngày | Nội dung | Việc cần làm |
|---|---|---|
| 1 | Chọn 3 project tốt nhất (khuyến nghị: `anchor-escrow`, `anchor-staking-pool`, `anchor-nft-minter`) | Viết lại README chuẩn cho từng repo: Mục tiêu, Kiến trúc (kèm sơ đồ account), Tech stack, Cách chạy local, Cách test. |
| 2 | Thêm CI/CD đơn giản | Thêm GitHub Actions chạy `anchor build` + `anchor test` tự động mỗi lần push — thể hiện tư duy engineering chuyên nghiệp. |
| 3 | Demo trực quan | Quay GIF/video ngắn (asciinema hoặc screen record) demo từng project, nhúng vào README. |
| 4 | Ôn tập tổng hợp | Đọc lại toàn bộ 11 tuần: file `comparison.md` (Bitcoin/Ethereum/Solana), checklist security `sealevel-attacks`, toàn bộ quiz đã làm. |
| 5 | Luyện 20 câu hỏi phỏng vấn (mục 2 bên dưới) | Tự trả lời thành tiếng (ghi âm), nghe lại, rút gọn câu trả lời cho gọn & tự tin. |
| 6 | Mock interview + live coding nhỏ | Tự đặt đề: "Viết instruction Anchor nhận thêm 1 tham số `amount`, kiểm tra `amount > 0`, transfer token." — làm trong 20-25 phút không xem tài liệu. |
| 7 | **Checkpoint cuối cùng** | Chạy qua Checklist kỹ năng (mục 3 bên dưới), chuẩn bị CV (gắn link 3 GitHub repo), apply. |

---

## PHỤ LỤC: TRACK SONG HÀNH — DSA BẰNG RUST (84 BÀI LEETCODE, 1 BÀI/NGÀY)

**Vì sao nên ghép thêm track này:** Phỏng vấn Intern Blockchain (dù chuyên Rust/Solana) gần như luôn có 1 vòng/đoạn hỏi thuật toán & cấu trúc dữ liệu cơ bản — và việc giải LeetCode bằng Rust còn có lợi ích phụ là làm bạn quen tay hơn với ownership/borrowing/Option/Result trong tình huống "thuật toán thuần", thứ rất khác với code Axum/Anchor hàng ngày. Track này được thiết kế để **chạy song song, không làm chậm** lộ trình Blockchain 12 tuần ở trên.

**Cách bố trí thời gian mỗi ngày (trong 4-6h đã có):**
- Dành **30-45 phút đầu ngày** cho 1 bài LeetCode — làm trước khi học Blockchain, coi như "khởi động não" mỗi ngày.
- Bài Hard được đánh dấu **(tuỳ chọn)** — nếu hôm đó lịch Blockchain nặng (đặc biệt tuần 8-10), có thể bỏ qua bài Hard mà không ảnh hưởng tiến độ chung.
- Mục tiêu không phải là "AC tất cả" mà là **nhận diện đúng pattern** (two pointers, sliding window, DFS/BFS, DP...) — đây là thứ interviewer thực sự đánh giá.

**Setup môi trường:** Tạo 1 repo riêng `rust-leetcode-84`, mỗi bài là 1 module (`day01_two_sum.rs`, `day02_contains_duplicate.rs`, ...), viết kèm `#[test]` cho từng bài bằng `cargo test`. Commit mỗi ngày 1 lần — chuỗi commit 84 ngày liên tục cũng là một tín hiệu tốt cho nhà tuyển dụng về tính kỷ luật.

> Lưu ý bản quyền: bảng dưới đây chỉ liệt kê **tên bài** trên LeetCode (kiến thức công khai, giống tên đề mục sách) để bạn tự tìm và giải; không sao chép đề bài/giải pháp của LeetCode.

### Tuần 1 — Array & Hashing (song hành Blockchain Fundamentals)
| Ngày | Bài LeetCode | Độ khó | Ghi chú pattern |
|---|---|---|---|
| 1 | Two Sum | Easy | Cặp số có tổng = target, dùng HashMap |
| 2 | Contains Duplicate | Easy | Kiểm tra phần tử trùng lặp bằng HashSet |
| 3 | Valid Anagram | Easy | Đếm ký tự bằng HashMap — liên hệ trực tiếp bài hash function hôm nay |
| 4 | Group Anagrams | Medium | Group string theo key đã sort |
| 5 | Top K Frequent Elements | Medium | HashMap đếm tần suất + sắp xếp |
| 6 | Product of Array Except Self | Medium | Prefix/suffix product, không dùng phép chia |
| 7 | Valid Sudoku | Medium | **Ôn tập tuần**: dùng nhiều HashSet cùng lúc |

### Tuần 2 — Two Pointers & Sliding Window (song hành Cryptography chuyên sâu)
| Ngày | Bài LeetCode | Độ khó | Ghi chú pattern |
|---|---|---|---|
| 8 | Valid Palindrome | Easy | Two pointers từ 2 đầu |
| 9 | Two Sum II – Input Array Is Sorted | Easy | Two pointers trên array đã sort |
| 10 | 3Sum | Medium | Sort + two pointers, tránh trùng lặp kết quả |
| 11 | Container With Most Water | Medium | Two pointers tối ưu diện tích |
| 12 | Best Time to Buy and Sell Stock | Easy | Sliding window 1 chiều |
| 13 | Longest Substring Without Repeating Characters | Medium | Sliding window + HashSet |
| 14 | Longest Repeating Character Replacement | Medium | **Ôn tập tuần**: sliding window nâng cao |

### Tuần 3 — Stack (song hành Bitcoin Architecture)
| Ngày | Bài LeetCode | Độ khó | Ghi chú pattern |
|---|---|---|---|
| 15 | Valid Parentheses | Easy | Stack cơ bản |
| 16 | Min Stack | Medium | Thiết kế stack hỗ trợ getMin O(1) |
| 17 | Evaluate Reverse Polish Notation | Medium | Dùng stack tính biểu thức — giống cách Bitcoin Script (stack-based) hoạt động |
| 18 | Generate Parentheses | Medium | Backtracking kết hợp stack logic |
| 19 | Daily Temperatures | Medium | Monotonic stack |
| 20 | Car Fleet | Medium | Stack + sắp xếp theo vị trí |
| 21 | Largest Rectangle in Histogram | Hard (tuỳ chọn) | **Ôn tập tuần**: monotonic stack nâng cao |

### Tuần 4 — Binary Search (song hành Ethereum Architecture)
| Ngày | Bài LeetCode | Độ khó | Ghi chú pattern |
|---|---|---|---|
| 22 | Binary Search | Easy | Binary search cơ bản |
| 23 | Search a 2D Matrix | Medium | Binary search trên matrix |
| 24 | Koko Eating Bananas | Medium | Binary search trên "đáp số" (answer space) |
| 25 | Find Minimum in Rotated Sorted Array | Medium | Binary search trên array bị xoay |
| 26 | Search in Rotated Sorted Array | Medium | Binary search tìm target trên array xoay |
| 27 | Time Based Key-Value Store | Medium | HashMap + binary search theo timestamp |
| 28 | Median of Two Sorted Arrays | Hard (tuỳ chọn) | **Ôn tập tuần**: binary search khó, có thể bỏ qua |

### Tuần 5 — Linked List (song hành Rust nâng cao cho on-chain)
| Ngày | Bài LeetCode | Độ khó | Ghi chú pattern |
|---|---|---|---|
| 29 | Reverse Linked List | Easy | Thao tác con trỏ cơ bản — luyện thêm cảm giác ownership trong Rust |
| 30 | Merge Two Sorted Lists | Easy | Merge 2 linked list |
| 31 | Reorder List | Medium | Tìm trung điểm + đảo chiều + merge |
| 32 | Remove Nth Node From End of List | Medium | Two pointers trên linked list |
| 33 | Add Two Numbers | Medium | Linked list mô phỏng phép cộng |
| 34 | Copy List with Random Pointer | Medium | HashMap để clone linked list có con trỏ phụ |
| 35 | Linked List Cycle | Easy | **Ôn tập tuần**: Floyd cycle detection |

### Tuần 6 — Trees nhập môn (song hành Solana Fundamentals)
| Ngày | Bài LeetCode | Độ khó | Ghi chú pattern |
|---|---|---|---|
| 36 | Invert Binary Tree | Easy | Duyệt cây cơ bản (DFS) |
| 37 | Maximum Depth of Binary Tree | Easy | DFS đệ quy |
| 38 | Same Tree | Easy | So sánh 2 cây bằng đệ quy |
| 39 | Subtree of Another Tree | Easy | DFS lồng nhau |
| 40 | Lowest Common Ancestor of a BST | Medium | Tính chất Binary Search Tree |
| 41 | Binary Tree Level Order Traversal | Medium | BFS theo từng level — liên hệ cách node Solana lan truyền block theo "level" |
| 42 | Validate Binary Search Tree | Medium | **Ôn tập tuần**: ràng buộc min/max khi duyệt |

### Tuần 7 — Trees nâng cao & Heap (song hành Anchor Framework)
| Ngày | Bài LeetCode | Độ khó | Ghi chú pattern |
|---|---|---|---|
| 43 | Binary Tree Right Side View | Medium | BFS lấy phần tử cuối mỗi level |
| 44 | Construct Binary Tree from Preorder and Inorder Traversal | Medium | Đệ quy + HashMap tra index |
| 45 | Kth Smallest Element in a BST | Medium | Duyệt in-order |
| 46 | Kth Largest Element in an Array | Medium | Min-Heap kích thước k (`BinaryHeap` trong Rust) |
| 47 | Last Stone Weight | Easy | Max-Heap cơ bản |
| 48 | K Closest Points to Origin | Medium | Heap theo khoảng cách |
| 49 | Top K Frequent Elements (làm lại) | Medium | **Ôn tập tuần**: kết hợp HashMap + Heap |

### Tuần 8 — Backtracking (song hành Escrow Program)
| Ngày | Bài LeetCode | Độ khó | Ghi chú pattern |
|---|---|---|---|
| 50 | Subsets | Medium | Backtracking sinh tập con |
| 51 | Combination Sum | Medium | Backtracking với điều kiện tổng |
| 52 | Permutations | Medium | Backtracking sinh hoán vị |
| 53 | Word Search | Medium | Backtracking trên grid + DFS |
| 54 | Letter Combinations of a Phone Number | Medium | Backtracking sinh chuỗi |
| 55 | Palindrome Partitioning | Medium | Backtracking + kiểm tra palindrome |
| 56 | N-Queens | Hard (tuỳ chọn) | **Ôn tập tuần**: backtracking kinh điển — tuần này Blockchain nặng (Escrow + security review), có thể bỏ qua bài này nếu thiếu thời gian |

### Tuần 9 — Graphs cơ bản (song hành SPL Token Program)
| Ngày | Bài LeetCode | Độ khó | Ghi chú pattern |
|---|---|---|---|
| 57 | Number of Islands | Medium | DFS/BFS trên grid |
| 58 | Clone Graph | Medium | DFS + HashMap clone node |
| 59 | Max Area of Island | Medium | DFS tính diện tích vùng liên thông |
| 60 | Pacific Atlantic Water Flow | Medium | Multi-source BFS/DFS |
| 61 | Course Schedule | Medium | Topological sort — liên hệ trực tiếp cách Solana lên lịch song song các transaction theo phụ thuộc account |
| 62 | Number of Connected Components in an Undirected Graph | Medium | Union-Find — liên hệ cách nhóm các account độc lập để chạy song song (Sealevel) |
| 63 | Redundant Connection | Medium | **Ôn tập tuần**: Union-Find tìm cạnh dư |

### Tuần 10 — Graphs nâng cao (song hành NFT Program)
| Ngày | Bài LeetCode | Độ khó | Ghi chú pattern |
|---|---|---|---|
| 64 | Course Schedule II | Medium | Topological sort trả về thứ tự đầy đủ |
| 65 | Network Delay Time | Medium | Dijkstra shortest path |
| 66 | Cheapest Flights Within K Stops | Medium | BFS/Bellman-Ford có giới hạn số bước |
| 67 | Min Cost to Connect All Points | Medium | Minimum Spanning Tree (Prim/Kruskal) |
| 68 | Word Ladder | Hard (tuỳ chọn) | BFS trên đồ thị từ — có thể bỏ qua nếu tuần này bận mint NFT |
| 69 | Alien Dictionary | Hard (tuỳ chọn) | Topological sort trên ký tự — có thể bỏ qua |
| 70 | Surrounded Regions | Medium | **Ôn tập tuần**: DFS/BFS xử lý biên grid |

### Tuần 11 — Dynamic Programming 1D (song hành DeFi/Staking)
| Ngày | Bài LeetCode | Độ khó | Ghi chú pattern |
|---|---|---|---|
| 71 | Climbing Stairs | Easy | DP cơ bản, kiểu Fibonacci |
| 72 | House Robber | Medium | DP 1D với ràng buộc không liền kề |
| 73 | House Robber II | Medium | DP 1D trên mảng dạng vòng |
| 74 | Longest Increasing Subsequence | Medium | DP O(n²) hoặc binary search O(n log n) |
| 75 | Coin Change | Medium | DP tối ưu số lượng "đồng xu" — liên hệ trực quan tới tối ưu hoá số lượng token |
| 76 | Maximum Subarray | Medium | DP / Kadane's algorithm |
| 77 | Word Break | Medium | **Ôn tập tuần**: DP trên chuỗi |

### Tuần 12 — DP 2D + Tổng ôn (song hành Portfolio & Interview Prep)
| Ngày | Bài LeetCode | Độ khó | Ghi chú pattern |
|---|---|---|---|
| 78 | Unique Paths | Medium | DP 2D trên grid |
| 79 | Longest Common Subsequence | Medium | DP 2D kinh điển |
| 80 | Edit Distance | Medium (tuỳ chọn) | DP 2D nâng cao — bỏ qua nếu cần thời gian polish portfolio |
| 81 | Merge Intervals | Medium | Ôn mix: sort + greedy, hay được dùng làm đề mock interview |
| 82 | Tự chọn 1 bài đã làm sai/chậm nhất trong 81 ngày trước | — | Làm lại không xem lời giải cũ |
| 83 | Mock interview: 1 bài Medium ngẫu nhiên, bấm giờ 35-40 phút | — | Mô phỏng đúng áp lực phòng phỏng vấn thật, không xem gợi ý |
| 84 | Tổng kết track DSA | — | Không làm bài mới — viết ra 5 pattern bạn còn yếu nhất để tiếp tục luyện sau khi đã đi làm |

---

## 1. BA PROJECT PORTFOLIO (chi tiết)

### 🟢 Project 1: `anchor-escrow` — Escrow / Token Swap Program
- **Mục tiêu thể hiện:** PDA, CPI sang SPL Token, security (signer check, has_one, checked arithmetic).
- **Kiến trúc:** `EscrowState` account (PDA) lưu thông tin initializer, mint A/B, amount mong muốn; `Vault` (token account do PDA làm authority) giữ token tạm.
- **Instructions bắt buộc:** `initialize_escrow`, `exchange`, `cancel`.
- **Test bắt buộc:** happy path, sai amount, người ngoài cancel (fail), double-exchange (fail).
- **Tiêu chí "production-grade":** không còn `unwrap()` tuỳ tiện, dùng `checked_*` cho mọi phép toán số học, README có sơ đồ account.

### 🟢 Project 2: `anchor-staking-pool` — DeFi Staking Program
- **Mục tiêu thể hiện:** Tính toán reward theo thời gian, quản lý state phức tạp hơn, hiểu DeFi primitive.
- **Kiến trúc:** `StakePool` (config: reward rate, total staked), `UserStake` (PDA theo từng user: staked_amount, reward_debt, last_update_timestamp).
- **Instructions bắt buộc:** `initialize_pool`, `stake`, `claim_rewards`, `unstake`.
- **Test bắt buộc:** stake → đợi → claim đúng số reward kỳ vọng, unstake một phần, double-claim không bị duplicate reward.
- **Tiêu chí "production-grade":** xử lý đúng trường hợp pool hết reward, không có integer overflow khi pool chạy lâu.

### 🟢 Project 3: `anchor-nft-minter` — NFT Minting Program (Metaplex)
- **Mục tiêu thể hiện:** CPI sang Metaplex Token Metadata program, hiểu chuẩn NFT, làm việc với metadata off-chain (URI/JSON).
- **Kiến trúc:** Instruction `mint_nft` tạo Mint (supply 1, decimals 0) + Metadata account (name, symbol, uri, creators, seller_fee_basis_points) + Master Edition.
- **Instructions bắt buộc:** `mint_nft` (tối thiểu); nâng cao: `update_metadata` (chỉ creator).
- **Test bắt buộc:** mint thành công, verify metadata đúng trên Explorer, người không phải creator không update được.
- **Tiêu chí "production-grade":** README có ảnh chụp NFT trên Solana Explorer devnet, giải thích rõ flow CPI.

> **Gợi ý đặt tên GitHub:** đặt cả 3 repo dưới 1 GitHub profile rõ ràng, pin 3 repo này lên trang chủ profile, mỗi repo có topic tag: `solana`, `anchor`, `rust`, `blockchain`.

---

## 2. 20 CÂU HỎI PHỎNG VẤN INTERN BLOCKCHAIN DEVELOPER (kèm gợi ý trả lời ngắn)

**Nhóm Fundamentals & Cryptography**
1. **Blockchain là gì, khác gì với database phân tán thông thường?** → Nhấn vào: immutability qua hash-chaining, không cần trusted third party, consensus.
2. **Hash function có những tính chất gì? Vì sao quan trọng với blockchain?** → Deterministic, one-way, avalanche effect, collision-resistant.
3. **Digital signature hoạt động như thế nào? Phân biệt với mã hoá (encryption)?** → Sign bằng private key, verify bằng public key, mục đích là xác thực + chống chối bỏ, không phải để giữ bí mật.
4. **Merkle Tree dùng để làm gì trong blockchain?** → Cho phép verify 1 transaction nằm trong block mà không cần tải toàn bộ block (SPV).

**Nhóm Bitcoin/Ethereum**
5. **UTXO model khác Account model như thế nào? Ưu nhược điểm mỗi loại?** → UTXO: privacy tốt hơn, parallel hoá dễ, nhưng quản lý state phức tạp hơn cho smart contract.
6. **EVM là gì? Gas dùng để làm gì?** → Máy ảo thực thi bytecode, gas để trả phí tính toán & chống spam/vòng lặp vô hạn.
7. **ERC-20 và ERC-721 khác nhau ở điểm gì?** → Fungible (interchangeable) vs Non-fungible (unique token id).

**Nhóm Solana cốt lõi**
8. **Vì sao Solana đạt throughput cao hơn Ethereum L1?** → Parallel execution (Sealevel) vì account đã khai báo rõ ràng dependency, PoH giúp leader sắp xếp nhanh, không cần global state lock.
9. **Account model của Solana hoạt động ra sao? Khác Ethereum thế nào?** → Mọi thứ là account (data hoặc executable), program là stateless, state tách riêng khỏi code — khác Ethereum (contract tự chứa storage).
10. **PDA (Program Derived Address) là gì, dùng để làm gì?** → Address được derive từ seeds + program_id, không có private key, cho phép program "ký" thay account đó qua `invoke_signed`.
11. **Rent trên Solana là gì?** → Phí lưu trữ data on-chain theo thời gian/dung lượng; account có thể "rent-exempt" nếu nạp đủ lamport tối thiểu.
12. **CPI (Cross-Program Invocation) là gì? Cho ví dụ thực tế.** → 1 program gọi instruction của program khác trong cùng transaction, ví dụ: program của bạn gọi SPL Token program để transfer token.
13. **Compute budget / compute unit là gì? Tại sao phải quan tâm khi viết program?** → Mỗi transaction có giới hạn compute unit, code không tối ưu (loop lớn, nhiều CPI) dễ bị fail do hết budget.

**Nhóm Anchor & Smart Contract Dev**
14. **Anchor giúp gì so với viết native Solana program?** → Tự sinh validation account (ownership, discriminator, signer), tự sinh IDL, macro giảm boilerplate, dễ test hơn.
15. **`#[account(init, payer, space)]` trong Anchor làm gì?** → Tự tạo account mới (CPI tới System Program), người trả phí rent, khai báo dung lượng cố định.
16. **Anchor dùng discriminator để làm gì?** → 8 byte đầu account để Anchor xác định đúng loại struct, tránh deserialize sai/account confusion attack.
17. **Nêu 3 lỗ hổng bảo mật phổ biến trong Solana program và cách phòng tránh.** → Missing signer check; integer overflow (dùng `checked_*`); account substitution/ownership confusion (luôn kiểm tra `owner`/`has_one`).

**Nhóm Token/NFT/DeFi**
18. **SPL Token khác ERC-20 ở điểm gì về kiến trúc?** → ERC-20: balance lưu trong storage của contract; SPL Token: balance lưu trong Token Account riêng của từng user, Mint account chỉ lưu metadata chung (supply, decimals, authority).
19. **NFT trên Solana được cấu trúc như thế nào (Metaplex)?** → Mint account (supply=1, decimals=0) + Metadata account (gắn qua PDA từ mint) chứa name/symbol/uri + Master Edition kiểm soát print/edition.
20. **AMM hoạt động theo công thức nào? Impermanent loss là gì?** → Constant product `x*y=k`; impermanent loss là tổn thất tạm thời khi giá 2 token trong pool lệch nhau so với lúc deposit, so với việc chỉ giữ (hold) token.

---

## 3. CHECKLIST KỸ NĂNG TRƯỚC KHI APPLY

**Kiến thức nền tảng**
- [ ] Giải thích được hash, Merkle tree, digital signature không cần nhìn tài liệu
- [ ] So sánh được UTXO vs Account model, nêu được ví dụ cụ thể
- [ ] Vẽ được sơ đồ kiến trúc Bitcoin / Ethereum / Solana trên giấy trong 5 phút

**Rust cho on-chain**
- [ ] Hiểu rõ Borsh serialization, vì sao Solana không dùng serde_json on-chain
- [ ] Biết dùng `checked_add`/`checked_sub`/`checked_mul` thay vì toán tử thường
- [ ] Đọc hiểu memory layout cơ bản (`#[repr(C)]`, alignment)

**Solana & Anchor**
- [ ] Tự viết được 1 Anchor program từ đầu (không copy template) trong < 1 giờ
- [ ] Hiểu rõ PDA, bump seed, và viết được `find_program_address`
- [ ] Hiểu CPI và đã từng tự code CPI gọi SPL Token program
- [ ] Biết debug bằng `anchor test`, đọc log lỗi (`msg!`, program log)
- [ ] Đã từng deploy program thật lên devnet (không chỉ chạy local)

**Bảo mật cơ bản (rất hay bị hỏi với intern Rust)**
- [ ] Biết tối thiểu 3 lỗ hổng phổ biến trong Solana program (xem repo `sealevel-attacks`)
- [ ] Code của bạn không có `unwrap()` bừa bãi ở chỗ có thể nhận input từ user

**Portfolio & Soft skill**
- [ ] Có 3 GitHub repo public, README rõ ràng, có test, có hướng dẫn chạy local
- [ ] Có ít nhất 1 program đã deploy & verify được trên Solana Explorer devnet
- [ ] Trả lời trôi chảy 20 câu hỏi phỏng vấn ở mục 2 mà không cần đọc lại note
- [ ] CV có link GitHub, ghi rõ tech stack: Rust, Anchor, Solana, SPL Token, Metaplex

---

## 4. GHI CHÚ SỬ DỤNG FILE EXCEL THEO DÕI TIẾN TRÌNH

File `Blockchain_Intern_Tracker.xlsx` đi kèm gồm các sheet:
- **Tổng quan**: thông tin chương trình, ngày bắt đầu, % hoàn thành tự động theo dữ liệu bạn nhập — bao gồm cả % Blockchain và % DSA theo từng tuần.
- **Theo dõi hàng ngày**: 84 ngày (12 tuần x 7 ngày), mỗi dòng gồm 2 nhóm cột song song:
  - Nhóm Blockchain: Chủ đề / Tài liệu / Nhiệm vụ thực hành / Trạng thái / Số giờ học / Ghi chú.
  - Nhóm DSA (mới): Chủ đề DSA / Bài LeetCode / Độ khó / Trạng thái DSA / Ghi chú DSA.
  Chỉ cần chọn dropdown "Trạng thái" và "Trạng thái DSA" mỗi ngày, % hoàn thành ở sheet Tổng quan tự cập nhật cho cả 2 track.
- **Checkpoint theo tuần**: 12 dòng checkpoint, tiêu chí đánh giá Blockchain + chủ đề DSA của tuần đó + Đạt/Chưa đạt.
- **Portfolio Projects**: 3 project với milestone con (ví dụ escrow có 7 milestone), tick từng milestone.
- **Câu hỏi phỏng vấn**: 20 câu, cột "Đã tự tin trả lời" để bạn tự đánh giá trước khi apply.

Cách dùng: mở file → vào sheet "Theo dõi hàng ngày" → mỗi ngày chọn dropdown "Trạng thái" (Blockchain) và "Trạng thái DSA" riêng biệt → quay lại sheet "Tổng quan" để xem % tiến độ của từng track.
