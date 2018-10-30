+++
date        = 2015-11-14
title       =  "Type safe OpenGL - Converting strings into types in D"
tags        = [ "D", "opengl" ]
+++

Writing glsl code is usually not that hard but it is easy to make mistakes, especially if you are changing some glsl files and forgetting to update the OpenGL calls.


We will parse a glsl shader and extract the types as a string at compile time, then we use those strings to create actual types. After we have obtained the types we will generate a wrapper for that glsl shader at compile time.

The benefit is that we will get a nice interface that won't allow you to make any mistakes. Another benefit will be that if the glsl shader changes the wrapper will change too, which helps you to keep the code in sync by generating helpful compiler errors.

In this blog post we will learn how we can turn strings into actual types at compile time.
```d
...
in vec2 fragTexCoord;
in vec3 fragNormal;
in vec3 fragVert;
...
["vec2","vec3","vec3"] to AliasSeq!(Vector2,Vector2,Vector3)
```

Converting strings to types is very easy in D
```d
template glslStringtoTypeV1(string stringType){
  static if(stringType == "int"){
    alias glslStringtoTypeV1 = int;
  }
  else static if(stringType == "float"){
    alias glslStringtoTypeV1 = float;
  }
}
void main()
{
  glslStringtoTypeV1!"float" f; //f is of type float
}
```

But [glsl has actually quite a few types](https://www.opengl.org/registry/doc/GLSLangSpec.4.40.pdf) and it would be annoying to  write that many `static if` blocks.

D also has template specializations like C++, maybe this would make the code easier to write?

```d
template glslStringtoTypeV2(string s){
  static assert(false, s ~ " is not a GLSL type");
}
template glslStringtoTypeV2(string s: "int"){
  alias glslStringtoTypeV2 = int;
}
template glslStringtoTypeV2(string s: "float"){
  alias glslStringtoTypeV2 = float;
}
void main()
{
  glslStringtoTypeV2!"float" f; //f is of type float
}
```
The nice thing about D is that we can pass almost any value into a template, in this case we use `string`. I think the code already looks much easier to write and read but it still has a lot of noise. Maybe we can reduce the noise with `template mixins`.

`Template mixins` allow you to expand a template with the `mixin` keyword.

```d
mixin template AddShader(T,string stringType){
  template glslStringtoTypeV2(string s: stringType){
    alias glslStringtoTypeV2 = T;
  }
}
template glslStringtoTypeV2(string s){
  static assert(false, s ~ " is not a GLSL type");
}
mixin AddShader!(int,"int");
mixin AddShader!(int,"float");
void main()
{
  glslStringtoTypeV2!"float" f; //err
}
```

This looks much better unfortunately this doesn't compile because `template mixins` expand into a different scope. D also allows us to insert strings at compile time with `mixins`. You can think of them like macros from C but they are more powerful because they are ordinary strings which can be manipulated.

```d
template AddShader(T,string stringType){
  enum AddShader = 
    "template glslStringtoTypeV2(string s: "~stringType.stringof~"){
         alias glslStringtoTypeV2 = " ~ T.stringof ~ ";" ~
    "}";
}
template glslStringtoTypeV2(string s){
  static assert(false, s ~ " is not a GLSL type");
}
mixin(AddShader!(int,"int"));
mixin(AddShader!(float,"float"));

void main()
{
  glslStringtoTypeV2!"float" f; //f is of type float
}
```
This is almost as nice as `template mixins` but I am not the biggest fan of writing source code as a string. Maybe there is still some room for improvement.

```d
struct ShaderType(T,string s){
  alias Type = T;
  enum string stringType = s;
}
enum isShaderType(T) = std.traits.isInstanceOf!(ShaderType,T);

template StringTypeGen(string stringType, ShaderTypes...)
  if(allSatisfy!(isShaderType, ShaderTypes)
     && ShaderTypes.length > 0)
{
  static if(ShaderTypes[0].stringType == stringType ){
    alias StringTypeGen = ShaderTypes[0].Type;
  }
  else static if(ShaderTypes.length == 1){
    static assert(false,stringType ~ " is not a recognized type.");
  }
  else{
    alias StringTypeGen = StringTypeGen!(stringType, ShaderTypes[1..$]);
  }
}
struct Matrix3{}
struct Vector3{}
alias glslStringToType(string s) = StringTypeGen!(s
    ,ShaderType!(int, "int")
    ,ShaderType!(float, "float")
    ,ShaderType!(double, "double")
    ,ShaderType!(uint, "uint")
    ,ShaderType!(bool, "bool")
    ,ShaderType!(Matrix3, "mat3")
    ,ShaderType!(Vector3, "vec3")
    //....
);
void main()
{
  glslStringToType!"float" f; //f is of type float
}
```

This is the final version which was a bit more work to create but it's easy to add glsl types, has nice error messages and is reusable. The last missing part is how we can turn a list of strings into a list of types.

```d
void main()
{
  alias VertexInput = AliasSeq!("mat3","int","vec3","double");
  alias VertexInputTypes = staticMap!(glslStringToType,VertexInput);
  //VertexInputTypes is of type AliasSeq!(Matrix3,int,Vector3,double)
}
```

