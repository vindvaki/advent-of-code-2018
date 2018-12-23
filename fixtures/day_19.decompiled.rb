def f()
    r = [1] + [0] * 5

    r[2] += 2
    r[2] *= r[2]
    r[2] *= 19
    r[2] *= 11
    r[1] += 2
    r[1] *= 22
    r[1] += 7
    r[2] += r[1]

    return r[0]  if r[0] > 8

    if r[0] > 0
        r[1] = 27    if r[0] <= 1
        r[1] *= 28   if r[0] <= 2
        r[1] += 29   if r[0] <= 3
        r[1] *= 30   if r[0] <= 4
        r[1] *= 14   if r[0] <= 5
        r[1] *= 32   if r[0] <= 6
        r[2] += r[1] if r[0] <= 7
        r[0] = 0     if r[0] <= 8
    end

    p r
    loop do
        r[3] = 1
        loop do
            r[5] = 1
            loop do
                if r[3] * r[5] == r[2]
                    r[0] += r[3]
                end
                r[5] += 1
                if r[5] > r[2]
                    break
                end
            end
            r[3] += 1
            if r[3] > r[2]
                return r[0]
            end
        end
    end
end

p f