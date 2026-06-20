use sha2::{Sha256, Digest};

// --- BƯỚC 1: CÁC HÀM BĂM (HASHING) ---

/// Hàm băm 1 khối dữ liệu bất kỳ ra 32 bytes
pub fn hash_block(data: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let mut hash = [0u8; 32];
    hash.copy_from_slice(&hasher.finalize());
    hash
}

/// Hàm băm gộp 2 node (trái + phải) trong Merkle Tree
pub fn hash_pair(left: &[u8; 32], right: &[u8; 32]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(left);
    hasher.update(right);
    let mut hash = [0u8; 32];
    hash.copy_from_slice(&hasher.finalize());
    hash
}

// --- BƯỚC 2: CẤU TRÚC DỮ LIỆU MERKLE TREE ---

#[derive(Debug, Clone, PartialEq)]
pub enum Direction {
    Left,  // Node anh em nằm bên trái
    Right, // Node anh em nằm bên phải
}

#[derive(Debug, Clone)]
pub struct ProofNode {
    pub hash: [u8; 32],
    pub direction: Direction,
}

pub struct MerkleTree {
    // Lưu trữ toàn bộ các tầng (layers). layers[0] là các giao dịch gốc.
    pub layers: Vec<Vec<[u8; 32]>>,
}

impl MerkleTree {
    /// Xây dựng Merkle Tree từ danh sách các giao dịch (transactions)
    pub fn new(transactions: &[&[u8]]) -> Self {
        if transactions.is_empty() {
            return MerkleTree { layers: vec![] };
        }

        let mut layers: Vec<Vec<[u8; 32]>> = Vec::new();
        
        // Tạo tầng 0 (Lá/Leaves) bằng cách băm từng giao dịch
        let leaves: Vec<[u8; 32]> = transactions.iter().map(|tx| hash_block(tx)).collect();
        layers.push(leaves);

        // Vòng lặp xây dựng các tầng trên cho đến khi chỉ còn 1 node (Root)
        while layers.last().unwrap().len() > 1 {
            let current_layer = layers.last().unwrap();
            let mut next_layer = Vec::new();

            // Duyệt từng cặp node ở tầng hiện tại
            for i in (0..current_layer.len()).step_by(2) {
                let left = &current_layer[i];
                
                // Nếu số lượng node lẻ, node cuối cùng sẽ tự ghép cặp với chính nó (nhân đôi)
                let right = if i + 1 < current_layer.len() {
                    &current_layer[i + 1]
                } else {
                    left
                };
                
                next_layer.push(hash_pair(left, right));
            }
            layers.push(next_layer);
        }

        MerkleTree { layers }
    }

    /// Lấy Merkle Root (Node duy nhất ở tầng cao nhất)
    pub fn root(&self) -> Option<[u8; 32]> {
        self.layers.last().and_then(|layer| layer.first().copied())
    }

    /// Sinh bằng chứng (Proof) cho một node ở vị trí `index` tại tầng 0
    pub fn get_proof(&self, mut index: usize) -> Vec<ProofNode> {
        let mut proof = Vec::new();

        if self.layers.is_empty() || index >= self.layers[0].len() {
            return proof; // Trả về mảng rỗng nếu index không hợp lệ
        }

        // Đi từ tầng dưới lên tầng áp chót
        for layer in self.layers.iter().take(self.layers.len() - 1) {
            let is_right_node = index % 2 == 1; // Node lẻ là node bên phải
            let sibling_index = if is_right_node { index - 1 } else { index + 1 };

            // Xử lý trường hợp node nằm ở cuối tầng và bị lẻ (sibling_index vượt mảng)
            let sibling_hash = if sibling_index < layer.len() {
                layer[sibling_index]
            } else {
                layer[index] // Theo quy tắc nhân đôi của Bitcoin
            };

            // Nếu node hiện tại bên phải, sibling của nó ở bên trái, và ngược lại
            let direction = if is_right_node { Direction::Left } else { Direction::Right };

            proof.push(ProofNode {
                hash: sibling_hash,
                direction,
            });

            // Lên tầng tiếp theo
            index /= 2;
        }

        proof
    }
}

// --- BƯỚC 3: HÀM XÁC MINH BẰNG CHỨNG ---

/// Xác minh xem một `leaf` có thực sự nằm trong cây có `expected_root` hay không
pub fn verify_proof(leaf: &[u8; 32], proof: &[ProofNode], expected_root: &[u8; 32]) -> bool {
    let mut current_hash = *leaf;

    for node in proof {
        match node.direction {
            Direction::Left => {
                // Sibling ở bên trái, nên hash_pair(sibling, current)
                current_hash = hash_pair(&node.hash, &current_hash);
            }
            Direction::Right => {
                // Sibling ở bên phải, nên hash_pair(current, sibling)
                current_hash = hash_pair(&current_hash, &node.hash);
            }
        }
    }

    // Kết quả sau khi hash liên tục lên tới đỉnh phải khớp với Root
    current_hash == *expected_root
}

// Hàm phụ trợ để in mảng byte ra chuỗi Hex
fn bytes_to_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

// --- BƯỚC 4: CHẠY THỬ (MAIN) ---

fn main() {
    // 1. Giả lập một danh sách giao dịch
    let tx1 = b"Alice pays Bob 10 BTC";
    let tx2 = b"Bob pays Charlie 5 BTC";
    let tx3 = b"Charlie pays Dave 2 BTC";
    // Thử với 3 giao dịch (số lẻ để test chức năng nhân đôi node cuối)
    let transactions: &[&[u8]] = &[tx1, tx2, tx3];

    println!("=== 1. XÂY DỰNG MERKLE TREE ===");
    let tree = MerkleTree::new(transactions);
    let root = tree.root().unwrap();
    println!("Merkle Root: {}", bytes_to_hex(&root));

    // 2. Lấy proof cho giao dịch thứ 2 (tx2, index = 1)
    let index_to_prove = 1;
    let leaf_to_prove = hash_block(transactions[index_to_prove]);
    
    println!("\n=== 2. SINH BẰNG CHỨNG (PROOF) CHO TX2 ===");
    println!("Dữ liệu: {:?}", String::from_utf8_lossy(transactions[index_to_prove]));
    println!("Hash của lá: {}", bytes_to_hex(&leaf_to_prove));
    
    let proof = tree.get_proof(index_to_prove);
    for (i, node) in proof.iter().enumerate() {
        println!("  Tầng {}: Sibling hash={}, Hướng={:?}", i, bytes_to_hex(&node.hash), node.direction);
    }

    // 3. Tiến hành xác minh
    println!("\n=== 3. XÁC MINH BẰNG CHỨNG ===");
    let is_valid = verify_proof(&leaf_to_prove, &proof, &root);
    println!("Xác minh giao dịch hợp lệ: {}", is_valid);

    // 4. Thử giả mạo giao dịch (Sửa tx2)
    println!("\n=== 4. TEST GIẢ MẠO GIAO DỊCH ===");
    let fake_tx = b"Bob pays Hacker 50 BTC";
    let fake_leaf = hash_block(fake_tx);
    let is_fake_valid = verify_proof(&fake_leaf, &proof, &root);
    println!("Xác minh giao dịch giả mạo: {}", is_fake_valid); // Sẽ trả về false
}