#include <stdio.h>
#include <string.h>
int main(){
  int sum=0;
  while(1){
    int i,map[520]={0};
    char in1[1000]={0};
    char in2[1000]={0};
    char in3[1000]={0};
    scanf(" %[^\n]",in1);
    scanf(" %[^\n]",in2);
    scanf(" %[^\n]",in3);
    int prio=0; 
    for(i=0;in1[i]!='\0';i++)
      map[in1[i]]=1;
    for(i=0;in2[i]!='\0';i++)
      map[in2[i]]=(map[in2[i]]==1)?2:map[in2[i]];
    for(i=0;in3[i]!='\0';i++)
      if(map[in3[i]]==2){
        prio=('a'<=in3[i] && in3[i]<='z')? in3[i]-'a'+1 : in3[i]-'A'+27;
        sum+=prio;
        break;
      }
    printf("%c %d %d\n",in3[i],prio,sum);
  }
  return 0;
}
