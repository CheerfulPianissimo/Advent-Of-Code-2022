let inpt = Object.values(process.argv).slice(2).join(' ').toString();
//let inpt = "bvwbjplbgvbhsrlpgdmjqwftvncz"
for(i=0;i<inpt.length-4;i++){
  let a=inpt.substr(i,14);
  let b=new Set(a)
  if(a.length==b.size){
    console.log(a)
    console.log(i+14)
    break;
  }
}
