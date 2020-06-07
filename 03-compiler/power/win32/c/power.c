int power(int a, int b){
    float sum ;
    int i;
    for (i = b; i > 0; i--)
    {
        sum*= a;
    }
    return sum;
}