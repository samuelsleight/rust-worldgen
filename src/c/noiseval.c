double generate_random_value(int x, int y, int seed) {
    int n = ((x * 157) + (y * 31337) + (seed * 2633)) & 0x7fffffff;
    n = (n << 13) ^ n;
    return (1.0 - ((n * (n * n * 15731 + 789221) + 1376312579) & 0x7fffffff) / 1073741824.0);    
}
