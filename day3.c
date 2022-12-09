#include <stdio.h>
#include <string.h>
int main(){
  int sum=0;
  while(1){
    int i,map[520]={0};
    char in[1000]={0};
    scanf(" %[^\n]",in);
    int len=strlen(in),prio=0; 
    for(i=0;i<len/2;i++)
      map[in[i]]+=1;
    for(;i<len;i++)
      if(map[in[i]]!=0){
        prio=('a'<=in[i] && in[i]<='z')? in[i]-'a'+1 : in[i]-'A'+27;
        sum+=prio;
        break;
      }
    printf("%c %d %d\n",in[i],prio,sum);
  }
  return 0;
}
