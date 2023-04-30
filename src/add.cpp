extern "C"
{
  int add(int a, int b)
  {
    return a + b;
  }

  struct myStruct {
    int a;
    float b;
    char c[10];
  };
  void fillStruct(struct myStruct* s)
  {
    s->a = 6;
    s->b = 12.23;
    s->c[0] = 'c';
    s->c[1] = 'a';
    s->c[2] = 't';
    s->c[3] = 0;
  }
}