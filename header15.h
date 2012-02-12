/*Author: Michael Barszcz
* Copyright Spring 2011
* This program solves a given board of the 15 puzzle as long as it is a valid board*/
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

#define N		4
#define BF		4
#define TRUE		1
#define FALSE		0

struct node {
  int a, b, c;
  struct node *next;
};

int flag;
char strategy[10];
struct node *start, *goal;
struct node **topen, **tclosed;
struct node **bopen, **bclosed;

struct node *initialize(), *expand(), *filter(), **merge();
int expand_a_node(), determine_h_val(), determine_g_val(), determine_f_val(), nodes_same(), compare_lists(), solvable(), determine_move();
void up(), dn(), lt(), rt(), print_a_node(), print_nodes();
void unpack_board(), pack_board();
void print_table();
int find_zero(), find_int(), count();
void remove_node();
