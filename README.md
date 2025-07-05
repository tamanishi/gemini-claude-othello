# Gemini-Claude Othello

> **Note**: This repository was created with generative AI assistance (Gemini and Claude).  
> **注意**: このリポジトリは生成AI（GeminiとClaude）の支援により作成されました。

**Language / 言語**: [English](#english) | [日本語](#japanese)

---

## English

A terminal-based Othello (Reversi) game written in Rust with three difficulty levels of CPU opponents.

### Features

- **Beautiful terminal UI** with colored disc display
- **Multiple game modes**: Player vs Player, Player vs CPU
- **3 CPU difficulty levels**:
  - **Easy**: Random moves with 0.8s thinking time
  - **Medium**: Greedy strategy with 1.2s thinking time  
  - **Hard**: Minimax algorithm with 2.0s thinking time
- **Smart CPU thinking simulation** with animated messages
- **Intuitive controls** with arrow keys and Enter/Space to place
- **Clear visual feedback** with optimized disc colors for terminal viewing

### Game Rules

Othello is a strategy board game for two players, played on an 8×8 board. Players take turns placing discs with their assigned color facing up. The objective is to have the majority of discs turned to display your color when the last playable empty square is filled.

- Players must place their disc adjacent to an opponent's disc
- All opponent discs between the new disc and existing player discs are flipped
- If a player cannot make a valid move, their turn is skipped
- The game ends when neither player can make a move
- The player with the most discs wins

### Installation

Make sure you have Rust installed on your system. If not, install it from [rustup.rs](https://rustup.rs/).

```bash
# Clone or download this repository
cd gemini-claude-othello

# Build the game
cargo build --release

# Run the game
cargo run
```

### Controls

#### Game Mode Selection
- `1` - Player vs Player
- `2` - Player vs CPU (leads to difficulty selection)
- `q` - Quit game

#### CPU Difficulty Selection
- `1` - Easy (Random moves)
- `2` - Medium (Greedy strategy)
- `3` - Hard (Minimax algorithm)
- `b` - Back to game mode selection
- `q` - Quit game

#### In-Game Controls
- `↑↓←→` - Move cursor
- `Enter` or `Space` - Place disc
- `q` - Quit to main menu

### Game Display

#### Board
- **●** (black) - Black player discs
- **●** (white) - White player discs
- **.** (white) - Empty squares
- Yellow background indicates cursor position

#### Information Area
- **Turn**: Shows current player with **◯** (black player) or **●** (white player)
- **Score**: Shows disc count as **◯: X | ●: Y**
- **Help**: Displays available controls

### CPU Algorithms

#### Easy Mode
- Selects moves randomly from all valid positions
- Quick decisions (0.8 seconds)
- Good for beginners learning the game

#### Medium Mode  
- Uses greedy strategy to maximize immediate disc gains
- Considers each move's immediate impact
- Moderate thinking time (1.2 seconds)

#### Hard Mode
- Implements minimax algorithm with alpha-beta pruning
- Looks 4 moves ahead
- Considers positional values and mobility
- Longest thinking time (2.0 seconds)
- Provides challenging gameplay for experienced players

### Technical Details

- **Language**: Rust 2021 Edition
- **Dependencies**: 
  - `crossterm` 0.29.0 - Terminal manipulation
  - `rand` 0.9.1 - Random number generation
- **Architecture**: Modular design with separate modules for game logic, board, players, and CPU AI

### Project Structure

```
src/
├── main.rs     # Main game loop and UI
├── game.rs     # Game logic and rules
├── board.rs    # Board representation
├── player.rs   # Player types and management
└── cpu.rs      # CPU AI algorithms
```

---

## Japanese

3段階の難易度を持つCPU対戦が可能な、Rustで書かれたターミナルベースのオセロ（リバーシ）ゲームです。

### 特徴

- **美しいターミナルUI** - カラフルなディスク表示
- **複数のゲームモード**: 対人戦、対CPU戦
- **3段階のCPU難易度**:
  - **Easy**: ランダム手選択（0.8秒思考時間）
  - **Medium**: グリーディー戦略（1.2秒思考時間）
  - **Hard**: ミニマックス法（2.0秒思考時間）
- **人間らしいCPU思考シミュレーション** - アニメーション付きメッセージ
- **直感的な操作** - 矢印キー + Enter/スペースキーで配置
- **明確な視覚フィードバック** - ターミナル表示に最適化されたディスク色

### ゲームルール

オセロは2人用の戦略ボードゲームで、8×8のボードで行います。プレイヤーは交互にディスクを配置し、相手のディスクを自分の色に変えることを目標とします。

- プレイヤーは相手のディスクに隣接してディスクを配置する必要があります
- 新しいディスクと既存のディスクの間にある相手のディスクはすべて裏返されます
- 有効な手がない場合、そのプレイヤーの手番はスキップされます
- 両プレイヤーが手を打てなくなったらゲーム終了
- より多くのディスクを持つプレイヤーの勝利

### インストール

システムにRustがインストールされていることを確認してください。未インストールの場合は[rustup.rs](https://rustup.rs/)からインストールしてください。

```bash
# このリポジトリをクローンまたはダウンロード
cd gemini-claude-othello

# ゲームをビルド
cargo build --release

# ゲームを実行
cargo run
```

### 操作方法

#### ゲームモード選択
- `1` - 対人戦
- `2` - 対CPU戦（難易度選択へ）
- `q` - ゲーム終了

#### CPU難易度選択
- `1` - Easy（ランダム手）
- `2` - Medium（グリーディー戦略）
- `3` - Hard（ミニマックス法）
- `b` - ゲームモード選択に戻る
- `q` - ゲーム終了

#### ゲーム中の操作
- `↑↓←→` - カーソル移動
- `Enter` または `Space` - ディスク配置
- `q` - メインメニューに戻る

### ゲーム表示

#### ボード
- **●**（黒色） - 黒プレイヤーのディスク
- **●**（白色） - 白プレイヤーのディスク
- **.**（白色） - 空きマス
- 黄色の背景はカーソル位置を示します

#### 情報エリア
- **ターン**: **◯**（黒プレイヤー）または **●**（白プレイヤー）で現在のプレイヤーを表示
- **スコア**: **◯: X | ●: Y** の形式でディスク数を表示
- **ヘルプ**: 利用可能な操作を表示

### CPUアルゴリズム

#### Easyモード
- 有効な手からランダムに選択
- 素早い判断（0.8秒）
- ゲームを学ぶ初心者に適しています

#### Mediumモード
- グリーディー戦略で即座の利得を最大化
- 各手の即座の影響を考慮
- 適度な思考時間（1.2秒）

#### Hardモード
- アルファベータ枝刈り付きミニマックス法を実装
- 4手先まで読み
- 位置価値とモビリティを考慮
- 最長の思考時間（2.0秒）
- 経験豊富なプレイヤーに挑戦的なゲームプレイを提供

### 技術詳細

- **言語**: Rust 2021 Edition
- **依存関係**: 
  - `crossterm` 0.29.0 - ターミナル操作
  - `rand` 0.9.1 - 乱数生成
- **アーキテクチャ**: ゲームロジック、ボード、プレイヤー、CPU AIの各モジュールによるモジュラー設計

### プロジェクト構造

```
src/
├── main.rs     # メインゲームループとUI
├── game.rs     # ゲームロジックとルール
├── board.rs    # ボード表現
├── player.rs   # プレイヤータイプと管理
└── cpu.rs      # CPU AIアルゴリズム
```

---

## License / ライセンス

This project is open source. Feel free to modify and distribute.

このプロジェクトはオープンソースです。自由に変更・配布してください。

## Acknowledgments / 謝辞

Initial implementation created with assistance from Gemini AI, enhanced and refined with Claude AI for improved user experience and advanced CPU algorithms.

初期実装はGemini AIの支援により作成され、ユーザーエクスペリエンスの向上と高度なCPUアルゴリズムのためにClaude AIにより機能強化・改良されました。