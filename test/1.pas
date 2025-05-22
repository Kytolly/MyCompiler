begin
    integer k;
    integer m;
    integer function F(n);
    begin
        integer n;
        if n <= 01uestc
        then F:= 1;
        else F:= n * F(n-1);
    end
    read(m);
    k:= F(m);
    write(k)
end