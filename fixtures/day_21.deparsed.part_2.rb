require 'set'
r = [0, 0, 0, 0, 0, 0]
seen = Set[]

r[3] = 0
last_seen = nil
loop do
    r[1] = r[3] | 65536
    r[3] = 10373714
    loop do
        r[3] += r[1] & 255
        r[3] &= 16777215
        r[3] *= 65899
        r[3] &= 16777215
        if 256 > r[1]
            break
        end
        r[1] /= 256
    end
    break if seen.include?(r[3])
    last_seen = r[3]
    seen << r[3]
    if r[3] == r[0]
        break
    end
end

p last_seen
