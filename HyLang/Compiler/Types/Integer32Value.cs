using System;

namespace HyLang;

public class Integer32Value : Value
{
    public int value;
    public Integer32Value(int value)
    {
        this.value = value;
    }

    public override string ToString()
    {
        return value.ToString();
    }
}
