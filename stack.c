/**
 * Stack operations using an array.
 * Name: Sooraj S, Roll No. 57, CSE S3 B2 2021-25
 */
#include <stdio.h>
#include <stdlib.h>

#define TYPE int
#define SIZE 20

typedef struct {
  TYPE data[SIZE];
  int top;
} stack;

stack init_stack() {
  stack a;
  a.top = -1;
  return a;
}

void push(stack *s, TYPE e) {
  if (s->top == SIZE - 1) {
    printf("Stack overflow!");
    exit(1);
  }
  s->data[++(s->top)] = e;
}

TYPE pop(stack *s) {
  if (s->top == -1) {
    printf("Stack underflow!");
    exit(1);
  }
  return s->data[(s->top)--];
}

void printstack(stack *s) {
  for (int i = 0; i <= s->top; i++)
    printf("%d ", s->data[i]);
  printf("\n");
}

void main() {
  stack s = init_stack();
  int ch, e;
  while (1) {
    printf("Stack Demo. Make your choice:\n 1. Push Element 2. Pop Element 3. Quit\n");
    scanf("%d", &ch);
    switch (ch) {
    case 1:
      printf("Enter the element\n");
      scanf("%d", &e);
      push(&s, e);
      break;
    case 2:
      e = pop(&s);
      printf("The element popped is %d\n", e);
      break;
    case 3:
      return;
    default:
      printf("Invalid choice!\n");
      continue;
    }
    printf("The stack is: ");
    printstack(&s);
  }
}
