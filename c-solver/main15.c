/*Michael Barszcz
* Copyright Spring 2011
* This program solves a given iteration of the 15 puzzle*/

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "header15.h"


int main(int argc, char **argv) {
  /*Check for correct input arguments*/
  int i;
  if (argc != 17) {
    printf("Usage: %s Numbers 0 to 15\n", argv[0]);
    return 1;
  }
  for (i=1; i<17; i++) {
    if (atoi(argv[i]) > 15 || atoi(argv[i]) < 0) {
      printf("Usage: %s Numbers 0 to 15\n", argv[0]);
      return 1;
    }
  }
    
  struct node *curr, *tsucc, *bsucc, *copen, *cbopen;
  int itr, tflag1, tflag2, bflag1, bflag2;
  itr = tflag1 = tflag2 = bflag1 =  bflag2 = 0;
  int cnt=0;

  start = initialize(argc, argv);
  if (flag == 1) {
    printf("Given board is unsolvable\n");
    return 0;
  }

  while (flag==0) {
  #pragma omp parallel
  {
    #pragma omp sections
    {
      #pragma omp section
      {
        /*Search from start board towards goal*/
        tsucc=expand(topen);
        tsucc=filter(tsucc, topen);
        tsucc=filter(tsucc, tclosed);
      }

      #pragma omp section
      {
        /*Search from goal board towards start*/
        bsucc = expand(bopen);
        bsucc = filter(bsucc, bopen);
        bsucc = filter(bsucc, bclosed);
      }
    }
  }
  tflag1 = compare_lists(tsucc, bopen);
  tflag2 = compare_lists(tsucc, bclosed);
  cnt = cnt + count(tsucc);
  if (tflag1!=1 && tflag2!=1)
    topen=merge(topen, tclosed, tsucc);


  bflag1 = compare_lists(bsucc, topen);
  bflag2 = compare_lists(bsucc, tclosed);
  cnt = cnt + count(bsucc);
  if (bflag1!=1 && bflag2!=1)
    bopen=merge(bopen, bclosed, bsucc);

  flag = tflag1 | tflag2 | bflag1 | bflag2;
  itr++;
  }
  printf("Board Solved! Iterations: %d   Nodes: %d\n", itr, cnt);

  return 0;
}

