#include <stdio.h>
int contains(int,int,int,int);
int main(){
  int cnt=0;
  while(1){
    int a,b,c,d;
    scanf("%d-%d,%d-%d",&a,&b,&c,&d);
    if(contains(a,b,c,d) || contains(c,d,a,b)
        || (b>=c && b<=d) || (a<=d && a>=c)){
      cnt++;
    }
    printf("%d\n ",cnt);
  }
  return 0;
}
//is a-b contained in c-d?
int contains(int a,int b,int c,int d){
  return a>=c && b<=d;
}
