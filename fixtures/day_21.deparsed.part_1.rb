def f(r)
    loop do
        r[3] = 123
        r[3] &= 456
        if r[3] == 72
            break
        end
    end

    r[3] = 0
    loop do
        r[1] = r[3] | 65536
        r[3] = 10373714
        loop do
            r[5] = r[1] & 255
            r[3] += r[5]
            r[3] &= 16777215
            r[3] *= 65899
            r[3] &= 16777215
            if 256 > r[1]
                # p r
                # fail
                break
            end
            # loop do
            #     r[4] = r[5] + 1
            #     r[4] *= 256
            #     if r[4] > r[1]
            #         break
            #     end
            #     r[5] += 1
            # end
            # (r[5] + 1) * 256 > r[1]
            r[5] = r[1] / 256
            r[1] = r[5]
            # p r
        end
        if r[3] == r[0]
            return
        end
    end
end

f([7967233, 0, 0, 0, 0, 0])