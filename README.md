# puzzle-elements-analysis
다양한 퍼즐 게임에서의 퍼즐 요소들을 분석합니다.

## 게임 장르별 변형 규칙

## 게임 장르 분석

(8개의 종류)
|Genre|Sokoban|Witness|Nikoli|Chess|Card Game|Factory|Board Game|multiplayer pencil game|
|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|
|Example|Baba is you, Isles of sky and sea, Maxwell's puzzling demon|The Witness, Funny Looksy, Taiji|14 variants minesweeper|Fairy Chess|Marvel Snap, Pokemon Pocket, Balatro, Poker, Holdem, Stackland|Shapez.io, block factory|boardgamearena|Math with bad drawings|
|SMT-solver available|---|---|---|---|---|---|---|---|
|directions needed|---|---|---|---|---|---|---|---|
|only adjaceny needed|---|---|---|---|---|---|---|---|
|available to upgrade to 3d or higher dimension|---|---|---|---|---|---|---|---|
|has undo, redo, reset funcionality|---|---|---|---|---|---|---|---|
|is multiplayer game that affected by game theory|---|---|---|---|---|---|---|---|
|can be placed at 3d location|---|---|---|---|---|---|---|---|
|should be inserted as form of minigame inside game|---|---|---|---|---|---|---|---|

1. Sokoban
- 당기기 가능?
- 여러개 밀기 가능?
- 좁은 공간에서 블록 미는 최적의 수 구하기(변형최소화) vs 더 다양한 기믹들 경험하기 vs 바바이즈유처럼 공간은 넓고 논리 기반 퍼즐 
3. Witness
4. Nikoli
- 
5. Chess
- 격자변형, 피스변형, 룰변형
6. Card
7. Factory
8. Board Game
9. multiplayer pencil game
- 공간
- 숫자
- 조합

// claude sonnet 3.5 suggest more genre for puzzle games:
1. **매치3**. Match-3/Tile-matching
- Games like Bejeweled, Candy Crush
- Focus on matching similar elements to clear them

2. **물리기반, 테트리스**. Physics-based puzzles, Block-falling/Tetris-like
- Games like Cut the Rope, Angry Birds
- Involve understanding and manipulating physics mechanics
- Tetris and its variants
- Focus on organizing falling pieces

3. **개방형 삼차원**. Portal-style/Spatial reasoning
- 삼차원 공간에서 이뤄지는 다양한 퍼즐게임들을 이쪽으로 분류했음. The Witness도 이쪽 계열로 볼 수 있지만, 선 잇기라는 독보적인 테마가 있으므로 따로 유지한다.
- Games like Portal, Antichamber
- Focus on understanding and manipulating space/geometry
- @me for additional, Manifold Garden, Superlimernal, ViewFinder

4. **방탈출**. Hidden object/Adventure puzzles, Escape room style
- Games like Room series
- Combining items and finding hidden clues
- Focus on environmental puzzles and sequential problem solving
- Often combine multiple puzzle types
- 
5. **언어퍼즐**. Word/Language puzzles
- Crosswords, Word Search, Wordle
- Focus on vocabulary and language manipulation

6. **패턴인식**. Pattern recognition
- Games like Simon Says
- Focus on memorizing and reproducing sequences
- 닌텐도 말랑말랑 두뇌게임의 스피디한 패턴인식, 와리오 미니게임 이런것도 어울릴 듯

7. **캐스트 퍼즐**. Mechanical puzzles
- Digital versions of physical puzzles like Rubik's Cube
- Focus on manipulating complex mechanisms
- 그 넣었다 뺐다 하는 그 퍼즐 말하는 것인듯.

8. **플랫포머**. Platformer Puzzle Game @me
- 윈도우 분할, 키보드 키 관련, 마우스 관련 등등 게임잼에서 참신한 아이디어를 갖고 플랫포머 퍼즐 게임으로 나오는 경우가 많다.
- 사이드뷰, 탑뷰 등 시점전환도 있으며, 슈퍼마리오 원더가 이쪽 계열의 교과서이다.

0. Logic grid puzzles
- 본인은 그냥 논리격자퍼즐들을 nikoli style로 분류했음.
- Games like Minesweeper
- Pure deduction based on given rules

## 게임 별 분석

### 1. 14가지 변형 지뢰찾기 (fandom)
[Q] - Quad: There must be at least 1 mine in every 2x2 area.

[C] - Connected: All mines are orthogonally or diagonally connected.

[T] - Triplet: Mines may not form a row of three orthogonally or diagonally.

[O] - Outside: All non-mines are orthogonally connected and all mines are orthogonally conected to the outside.

[D] - Dual: All mines must form 1x2 or 2x1 blocks. Blocks do not touch each other.

[S] - Snake: All mines form a single "snake" whose body does not touch itself. (All mines are connected orthogonally forming some sort of trail or "snake")

[B] - Balance: The number of mines in each row and column is the same.

[M] - Multiple: Each mine in a colored cell counts as two (Remaining mine count is unaffected by this rule). [Checkerboard coloring]

[L] - Liar: Each clue is either one greater or one less than the actual value.

[W] - Wall: The clue indicates the lengths of consecutive mines in the neighboring 8 cells.

[N] - Negation: The clue indicates the difference in the number of mines between adjacent colored and uncolored cells. [Checkerboard coloring]

[X] - Cross: The clue indicates the number of mines in a cross region (+) within 2 tiles distance.

[P] - Partiton: The clue indicates the number of consecutive groups of mines in the neighboring 8 cells.

[E] - Eyesight: The clue indicates the number of non-mines it can see in 4 directions (including itself). A mine in the way blocks the sight.

### 2. isles of sky and sea
밀 수 있는 오브제 - 박스, 파란박스, 물블록, 돌, 물, 불, 회오리, 눈덩이, 화염박스
바닥 오브제 - 물, 얕은 물, 구덩이, 구덩이낙엽, 까시, 용암, 마그마, 빙판, 높이 반블록
얹혀지는 오브제 - 벽, 열쇠벽, 포탈, 종과 울타리, 불, 바위, 물길, 수정, 일방향 숏컷 해금블록, 발판과 울타리, 화산 터지는 얼굴 블록
기타 오브제 - 오브, 별, 열쇠, 밀물썰물, 연꽃잎
심화 - 동기화/반동기화 블록/주인공, 중첩된 검은 안개(블록)
interesting - 토네이도.

### 3. Baba is you
baba has baba never die

wall rock water door key box robot belt
stop push pull sink float hot warm shut open move shift
baba, keke, me
you, win, death, text

tree, fungus, bug, skull, jelly, rose, violet

is, and, not

not ~ is ~
A is A

### 4. Maxwell's puzzling demon
