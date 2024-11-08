# puzzle-elements-analysis
다양한 퍼즐 게임에서의 퍼즐 요소들을 분석합니다.

14가지 변형 지뢰찾기 (fandom)
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

isles of sky and sea
밀 수 있는 오브제 - 박스, 파란박스, 물블록, 돌, 물, 불, 회오리, 눈덩이, 화염박스
바닥 오브제 - 물, 얕은 물, 구덩이, 구덩이낙엽, 까시, 용암, 마그마, 빙판, 높이 반블록
얹혀지는 오브제 - 벽, 열쇠벽, 포탈, 종과 울타리, 불, 바위, 물길, 수정, 일방향 숏컷 해금블록, 발판과 울타리, 화산 터지는 얼굴 블록
기타 오브제 - 오브, 별, 열쇠, 밀물썰물, 연꽃잎
심화 - 동기화/반동기화 블록/주인공, 중첩된 검은 안개(블록)
interesting - 토네이도.
