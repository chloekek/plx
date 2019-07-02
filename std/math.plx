namespace

    std;

features

    value

        log;

    with description

        "log b x approximates the base b logarithm of x.";

    of type

        float -> float -> float;

    is defined as

        over b x abstract
            ln x / ln b;

    end value;

end namespace;
