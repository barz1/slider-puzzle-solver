/*Author: Michael Barszcz
* Copyright Spring 2011
* This program solves a given board of the 15 puzzle as long as it is a valid board*/

#include "header15.h"
struct node *initialize(int argc, char **argv) {

  /*Initialize Values for Goal Board*/
  int arr_a[N+1][N] = {{1, 2, 3, 4}, {5, 6, 7, 8}, {9, 10, 11, 12}, {13, 14, 15, 0}, {0, 0, 0, 0}};
  int arr_b[N+1][N] = {0};

  /*Initialize starting board*/
  int idx=1, i, j;
  for (i=0; i<N; i++) {
    for (j=0; j<N; j++) {
      arr_b[i][j] = atoi(argv[idx++]);
    }
  }

  /*Check if given board is solvable*/
  if (!solvable(arr_b)) {
    flag=1;
    return NULL;
  }

  /*Decide what to store as move for start and goal board, shouldnt be 0-3 since these have meaning
   *Initialize Start Board*/
  struct node *tp = (struct node *)malloc(sizeof(struct node));
  arr_b[N][2] = determine_h_val(arr_b);
  pack_board(tp, arr_b);
  tp->next = NULL;
  start = tp;

  /*Initialize Goal Board*/
  struct node *bp=(struct node *)malloc(sizeof(struct node));
  arr_a[N][1] = determine_g_val(arr_a);
  pack_board(bp, arr_a);
  goal = bp;
  bp->next = NULL;

  /*Print start and goal board*/
  printf("\nInitial State\n");
  print_a_node(start);
  printf("\n\nGoal State\n");
  print_a_node(goal);
  printf("\n");

  /*Allocate space for topen, tclosed, bopen, bclosed*/
  topen = (struct node **)calloc(1024,sizeof(struct node *));
  tclosed = (struct node **)calloc(1024,sizeof(struct node *));
  bopen = (struct node **)calloc(1024,sizeof(struct node *));
  bclosed = (struct node **)calloc(1024,sizeof(struct node *));


  int fval = determine_f_val(start);
  topen[fval] = start;
  fval = determine_f_val(goal);
  bopen[fval] = goal;
  return start;
  
}

/*Function to expand current board to 4 possible moves*/
struct node *expand(struct node **list) {
  int i, j, k, cnt, indx;
  struct node *cp, *tp, *expanded;
  int cur_brd[N+1][N] = {0};
  int succ_buf[BF][N+1][N] = {0};

  i=0;
  /*Move to list in open with smallest fval*/
  while (list[i] == NULL) {
    i+=1;
  }
  expanded = list[i];
  unpack_board(expanded, cur_brd);
  /*Call expand_a_node to return 3D array of possible next moves*/
  cnt = expand_a_node(cur_brd, succ_buf);
  tp = malloc(sizeof(struct node));
  cp = tp;
  cp->next = malloc(sizeof(struct node));

  /*Set g and h values for newly generated boards*/
  for (i=0; i<cnt; i++) {
    succ_buf[i][N][2] = determine_h_val(succ_buf[i]);
    succ_buf[i][N][1] = determine_g_val(succ_buf[i]);
    pack_board(cp, succ_buf[i]);
    if (i != cnt-1) {
      cp=cp->next;
      cp->next = malloc(sizeof(struct node));
    }
  }
  cp->next = NULL;
  /*Return pointer to first board in list of successor boards*/
  return tp;
}

/*Function to expand current board into four possible successors*/
int expand_a_node(int node[N+1][N], int succ_buf[BF][N+1][N]) {
  int i, j, cnt, found, g_val, h_val, y, z;
  i = find_zero(node)/N;
  j = find_zero(node)%N;
  cnt = 0;

  /*Check if down move is within bounds*/
  if (i+1 < N) {
    for (y=0; y<N; y++) {
      for (z=0; z<N; z++) {
        succ_buf[cnt][y][z] = node[y][z];
      }
    }
    dn(i, j, cnt, succ_buf);
    succ_buf[cnt][N][3] = 0;
    cnt+=1;
  }

  /*Check if right move is within bounds*/
  if (j+1 < N) {
    for (y=0; y<N; y++) {
      for (z=0; z<N; z++) {
        succ_buf[cnt][y][z] = node[y][z];
      }
    }
    rt(i, j, cnt, succ_buf);
    succ_buf[cnt][N][3] = 1;
    cnt+=1;
  }

  /*Check if up move is within bounds*/
  if (i-1 >= 0) {
    for (y=0; y<N; y++) {
      for (z=0; z<N; z++) {
        succ_buf[cnt][y][z] = node[y][z];
      }
    }
    up(i, j, cnt, succ_buf);
    succ_buf[cnt][N][3] = 3;
    cnt+=1;
  }

  /*Check if left move is within bounds*/
  if (j-1 >= 0) {
    for (y=0; y<N; y++) {
      for (z=0; z<N; z++) {
        succ_buf[cnt][y][z] = node[y][z];
      }
    }
    lt(i, j, cnt, succ_buf);
    succ_buf[cnt][N][3] = 2;
    cnt+=1;
  }

  return cnt;
}

int determine_h_val(int board[N+1][N]) {
  int goal_index, goal_i, goal_j, distance, total, i, j;
  distance=0;

  /*Loop to determine h value based on abs(goal_i-curr_i)+abs(goal_j-curr_h)*/
  for (i=0; i<N; i++) {
    for (j=0; j<N; j++) {
      if (board[i][j] == 0)
        goal_index=(N*N)-1;
      else
        goal_index=board[i][j]-1;
      goal_i = goal_index/N;
      goal_j = goal_index%N;
      distance = distance + (int)fabs(goal_i-i) + (int)fabs(goal_j-j);
    }
  }
  return distance;
}

int determine_g_val(int board[N+1][N]) {
  int start_index, start_i, start_j, distance, idx, i, j;
  idx=distance=0;

  for (i=0; i<N; i++) {
    for (j=0; j<N; j++) {
      start_index = find_int(board[i][j], start);
      start_i = start_index/N;
      start_j = start_index%N;
      distance = distance + (int)fabs(start_i-i) + (int)fabs(start_j-j);
    }
  }
  return distance;
}

int determine_f_val(struct node *selected) {
  int cur_g, cur_h, cur_bit_g, cur_bit_h, g, h, cnt, n;
  cnt=2;
  g = h  = 0;

  /*Pull g and h value out of node->c*/
  for (n=0; n<8; n++) {
    cur_bit_g = (unsigned int)(selected->c>>(cnt+8))&1;
    cur_bit_h = (unsigned int)(selected->c>>cnt)&1;
    g |= (cur_bit_g<<n);
    h |= (cur_bit_h<<n);
    cnt+=1;
  }
  return (11*g)+(3*h);
}

int determine_move(struct node *selected) {
  int cur_mov, cur_mov_bit, n;
  cur_mov = cur_mov_bit = 0;
  for (n=0; n<2; n++) {
    cur_mov_bit = (unsigned int)(selected->c>>n)&1;
    cur_mov |= (cur_mov_bit<<n);
  }
  return cur_mov;
}

/*Swap open space[i][j] with number at position[i-1][j]*/
void up(int i, int j, int index, int succ_buf[BF][N+1][N]) {
  int temp_int = succ_buf[index][i-1][j];
  succ_buf[index][i-1][j] = 0;
  succ_buf[index][i][j] = temp_int;
}

/*Swap open space[i][j] with number at position[i+1][j]*/
void dn(int i, int j, int index, int succ_buf[BF][N+1][N]) {
  int temp_int = succ_buf[index][i+1][j];
  succ_buf[index][i+1][j] = 0;
  succ_buf[index][i][j] = temp_int;
}

/*Swap open space[i][j] with number at position[i][j-1]*/
void lt(int i, int j, int index, int succ_buf[BF][N+1][N]) {
  int temp_int = succ_buf[index][i][j-1];
  succ_buf[index][i][j-1] = 0;
  succ_buf[index][i][j] = temp_int;
}

/*Swap open space[i][j] with number at position[i][j+1]*/
void rt(int i, int j, int index, int succ_buf[BF][N+1][N]) {
  int temp_int = succ_buf[index][i][j+1];
  succ_buf[index][i][j+1] = 0;
  succ_buf[index][i][j] = temp_int;
}

struct node *filter(struct node *succ_list, struct node **list) {
  struct node *curr_succ = succ_list;           /*Pointer to head of succ list*/
  int f_val;
  int flg=1;
  struct node *curr_hp;
  while (curr_succ != NULL) {
    f_val = determine_f_val(curr_succ);
    curr_hp = list[f_val];                      /*Get list from open[fval]*/
    while (curr_hp != NULL && flg == 1) {       /*Compare succ to curr open list*/
      if (nodes_same(curr_hp, curr_succ)) {
        struct node *temp = curr_succ;
        curr_succ=curr_succ->next;
        remove_node(&succ_list, temp);
        flg=0;
      }
      else                              /*If not duplicate move to next node*/
        curr_hp=curr_hp->next;

    }
    if (flg == 1)               /*Flag=1 if no duplicate found, Flag=0 if duplicate found*/
      curr_succ=curr_succ->next;

    flg=1;
  }
  /*Return new successor list*/
  return succ_list;
}

struct node **merge(struct node **hp, struct node **cp, struct node *succ) {

  /*Merge current open node to closed list*/
  int i=0;
  while (hp[i] == NULL) {
    i++;
  }

  struct node *curr_list = hp[i];
  hp[i] = hp[i]->next;
  curr_list->next = cp[i];
  cp[i] = curr_list;

  struct node *curr_succ = succ;
  while (succ != NULL) {
    int fval = determine_f_val(curr_succ);
    struct node *curr_list = hp[fval];
    succ = succ->next;
    curr_succ->next = hp[fval];
    hp[fval] = curr_succ;
    curr_succ = succ;
  }

  return hp;
}

/*Function to find a given integer 1-16 in board*/
/*As of 6/2/2011 see about changing to format fo find_zero(), may not work due to where each function is called*/
int find_int(int to_find, struct node *selected) {
  int index=0, i, j;
  int brd[N+1][N] = {0};
  unpack_board(selected, brd);
  for (i=0; i<N; i++ ) {
    for (j=0; j<N; j++) {
      if (brd[i][j] == to_find)
        return index;

      index+=1;
    }
  }
  return -1;
}

/*Function to find zero (blank space) within a board*/
int find_zero(int board[N+1][N]) {
  int i,j, index, flag;
  index = 0;
  for (i=0; i<N; i++) {
    for(j=0; j<N; j++) {
      if (board[i][j] == 0) {
        return index;
      }
      index+=1;
    }
  }
  return -1;
}

/*Function to print a list of nodes*/
void print_nodes(struct node *cp) {
  while (cp!=NULL) {
    print_a_node(cp);
    cp=cp->next;
  }
}

/*Function to print a single node */
void print_a_node(struct node *np) {
  int i, j, cnt, n, cur_mag;
  unsigned int cur_bit;
  int brd[N+1][N] = {0};
  cnt=0;
  unpack_board(np, brd);
  for (i=0; i<=N; i++) {
    for(j=0; j<N; j++) {
      printf("%2d ", brd[i][j]);
    }
    printf("\n");
  }
  printf("\n");
}

/*Function to compare two nodes*/
int nodes_same(struct node *current, struct node *comp) {
  if (current->a != comp->a)
      return FALSE;
  if (current->b != comp->b)
     return FALSE;

  return TRUE;
}

/*Function to print number of nodes at each hash index, useful to see how efficient hashing algorithm is*/
void print_table() {
  int i=0;
  struct node *curr;
  while (i<1024) {
    int count=0;
    curr = tclosed[i];
    //printf("Current nodes at table index %d\n", i);
    while (curr != NULL) {
      count+=1;
      //print_a_node(curr);
      curr = curr->next;
    }
    printf("Node count at %d: %d\n", i, count);
    i+=1;
  }
}

/*Function to pack a 2D array into a struct node, think about changing implementation to return type node instead of void 6/2/2011*/
void pack_board(struct node *selected, int board[N+1][N]) {
  int pack_a, pack_b, pack_c, cnt, i, j, n;
  unsigned int cur_bit_a, cur_bit_b, cur_bit_c;
  cnt = pack_a = pack_b = pack_c = 0;
  for(i=0; i<2; i++) {
    for (j=0; j<4; j++) {
      for(n=0; n<4; n++) {
        cur_bit_a = (unsigned int)(board[i][j]>>n)&1;
        cur_bit_b = (unsigned int)(board[i+2][j]>>n)&1;
        pack_a |= (cur_bit_a << cnt);
        pack_b |= (cur_bit_b << cnt++);
      }
    }
  }
  selected->a = pack_a;
  selected->b = pack_b;
  cnt=0;
  i=3;
  /*Pack move into two bits*/
  cur_bit_c = 0;
  for (n=0; n<2; n++) {
    cur_bit_c = (unsigned int)(board[N][i]>>n)&1;
    pack_c |= (cur_bit_c << cnt++);
  }
  i-=1;
  /*Pack g and h value, 8 bits each*/
  while (i>0) {
    for (n=0; n<8; n++){
      cur_bit_c = (unsigned int)(board[N][i]>>n)&1;
      pack_c |= (cur_bit_c << cnt++);
    }
    i-=1;
  }
  selected->c = pack_c;
}

/*Function to unpack node into 2D array, again, think about returning 2D array instead of void*/
void unpack_board(struct node *selected, int board[N+1][N]) {
  int cnt=0, i, j, n;
  for (i=0; i<=N; i++) {
    for (j=0; j<N; j++) {
      board[i][j]=0;
    }
  }
  board[N][3]=0;
  int cur_bit_a, cur_bit_b, cur_bit_c;
  for (i=0; i<2; i++) {
    for (j=0; j<N; j++) {
      for (n=0; n<4; n++) {
        cur_bit_a = (unsigned int)(selected->a>>cnt)&1;
        cur_bit_b = (unsigned int)(selected->b>>cnt)&1;
        board[i][j] |= (cur_bit_a << n);
        board[i+2][j] |= (cur_bit_b << n);
        cnt+=1;
      }
    }
  }
  cnt=0;
  i=3;
  cur_bit_c = 0;
  for (n=0; n<2; n++) {
    cur_bit_c = (unsigned int)(selected->c>>n)&1;
    board[N][i] |= (cur_bit_c << n);
    cnt+=1;
  }
  i-=1;
  while (i>0) {
    for (n=0; n<8; n++) {
      cur_bit_c = (unsigned int)(selected->c>>cnt)&1;
      board[N][i] |= (cur_bit_c << n);
      cnt +=1;
    }
    i-=1;
  }

}

/*Function to compare succ list to current open/closed list, used to check if solution has been found*/
int compare_lists(struct node *succ_list, struct node **hp) {
  struct node *curr = succ_list;
  struct node *curr_hp;

  while (curr != NULL) {
    int fval = determine_f_val(curr);
    curr_hp = hp[fval];
    while (curr_hp != NULL) {
      if (nodes_same(curr, curr_hp)) 
        return 1;
      else
        curr_hp = curr_hp->next;
    }
    curr = curr->next;
  }
  return 0;
}

/*Function to remove a node from list*/
void remove_node (struct node **hp, struct node *dupl) {
  struct node *current = *hp;
  int flg=1;

  /*Check if node to be removed is head node */
  if (nodes_same(current, dupl)) {
    struct node *temp = current;
    *hp = (*hp)->next;
    free(temp);
    flg=0;
  }

  /*Otherwise move through the list to find node */
  while (current->next != NULL && flg) {
    if (nodes_same(current->next, dupl)) {
      struct node *temp = current->next;
      current->next = current->next->next;
      temp->next = NULL;
      free(temp);
      flg=0;
    }
    else
      current = current->next;
  }
}

/*Function to count the number of nodes in a list*/
int count(struct node *list) {
  struct node *curr = list;
  int count = 0;
  while (curr != NULL) {
    count+=1;
    curr = curr->next;
  }
  return count;
}

/*Function to determine if given board is solvable or not*/
int solvable(int board[N+1][N]) {
  int idx, inv_num, cnt, scnt;
  inv_num=0;

  for (cnt=0; cnt<N*N; cnt++) {
    int cur_i = cnt/N;
    int cur_j = cnt%N;
    idx = (cur_i*4)+cur_j+1;    /*Set index to current index+1*/
    for (cnt=idx; scnt<N*N; scnt++) {
      int new_i = scnt/N;
      int new_j = scnt%N;
      if (board[new_i][new_j]>0 && board[cur_i][cur_j] > board[new_i][new_j])
        inv_num+=1;
    }
  }

  int zero_i=find_zero(board)/N;

  if (zero_i%2==0 && inv_num%2 != 0)
    return TRUE;
  else if (zero_i%2 != 0 && inv_num%2 == 0)
    return TRUE;
  else
    return FALSE;
}

