---
layout: post
title: "Unreal Engine 4 vs Unity 5 (Part 1)"
description: ""
category:
tags: []
---

#Debugging
Debugging is very important for me, I want to know every thing that happens in my application.
In UE4 you can just hit F5 and start the editor in debug mode. You can still change the code but unfortunately you can not compile in VS while you are debugging. Luckily you can compile the game code within the editor which makes development a breeze because you can set breakpoints at any time that you like and it just works.

Debugging even works in a networked game which is super awesome. Consider the following scenario, 4 clients one server and you set a breakpoint at a function that is called on all clients.

The breakpoint will trigger 4 times. At any point you can inspect any value that you like.

Unity has also the possibility to debug game code but it is not as streamlined as in UE4. There is a plugin called 'UnityVS' which can attach itself to either the editor or to a standalone game. Unfortunately the debugger is very slow on my machine, if I attach UnityVS to the editor and hit play it will take around 10 seconds to enter the playmode.

Also it seems you can only debug one instance at a time which is a bit annoying if you are creating a networked game. UnityVS doesn't allow you to edit the code while you are debugging.

#Stability
This is actually a very strange point, I would consider Unity as well as UE4 to be somewhat unstable. It only took me 2 days to corrupt my project in Unity and I wasn't able to fix it easily. After a few hours of fiddling around I was able to find the culprit. I had written myself a custom tool that built a standalone game inside the asset folder which results in some very strange error messages to say the least.
Unity also crashed several times while I was developing my editor script.

UE4 has frequent editor crashes and the hot reloading system is not very reliable. Changes to blueprints sometimes just don't get detected which means that you have to restart the editor very often. The same thing applies to C++ changes but at least you get a visual representation that something is wrong because the compile button in the editor will have disappeared.

UE4 always crashes when you mess up. For example if you forgot to do a nullptr check and you access a nullptr the editor will crash.You should always be in debug mode because you get very weird messages if you are not.

I haven't had a code crash in Unity so far. If you access a nullptr in Unity it will just log it.

#Project Upgrades
This is a major downside for UE4. Every time Epic release a minor version upgrade something breaks in some unexpected ways. My only recommendation is too wait and stay at least one version behind. For example if 4.9 is released you should upgrade from 4.7 to 4.8. At least by then the community will have found some common issues. Also you should probably never upgrade to a newly released version like 4.8.0 unless it includes a must have feature. Those versions are usually very
buggy.

I recently upgraded from 4.7.6 to 4.8.1 and I only recognized that the compile times were much slower after a few weeks and switching back to 4.7.6 would require some changes.[Link to the forum post](https://forums.unrealengine.com/showthread.php?75217-Slower-build-times-in-4-8)
The problem was that Epic forgot to integrate some changes from 4.7 to 4.8 which results in longer compile times. They went up from 7sec to 18sec on my machine.

Unfortunately I have never upgraded a project in Unity but I would assume that this only happens in major version upgrades.

#Accessible Source
Open/Accessible source is very important for me because if I have a problem I need to solve it myself. It is also very helpful to understand how the API is working. To be very honest I don't even have the whole source code of UE4 on my machine because those millions lines of C++ in Visual Studio is just too much for my tiny processor.

But it is very nice to have the source of the gameplay framework and I am constantly browsing though it.

Unity is the definition of closed source. I know that they have some open source projects on Bitbucket but that is pretty much it. I have never seen such a closed environment.

Let me give you a small example. Unity recently released UNET, a multilayer solution for Unity. They ship with scripts like a NetworkManagerHUD which is a prototyping script that gives you a simple GUI and allows you to start a server and connect as a client to a server. Even the implementation of such a simple script is hidden from you which is very strange because it would serve as a great example.

#Coding
UE4 uses a custom C++ solution, you basically write C++ with a lot of macros and those macros produce some code that gets picked up by the UE4 Header Tool which generates some code.

UE4 has some gotchas that you need to look out for. For example if you write

{% highlight c++ %}
UFooComponent* FooComponent;
....

FooComponent = CreateDefaultSubComponent(TEXT("Foo"));
{% endhighlight %}

This pointer will be reset to null at one point. You have to annotate such a pointer with a macro like so.

{% highlight c++ %}
UPROPERTY()
UFooComponent* FooComponent;
{% endhighlight %}

Unity comes with a custom C# soltuion which is far less weird than in UE4. I know for a fact that they are doing IL rewriting in UNET and I am pretty sure that they do the same thing with MonoBehaviors because I can not find the Start method.

In Unity you can just write

{% highlight c# %}
[SyncVar]
float health;
{% endhighlight %}

and it will synchronize the value from the server to the clients.

The same thing would look like this in UE4:

{% highlight c++ %}
UPROPERTY(Replicated,Reliable)
float Health;
...
DOREPLIFETIME(AFooSomething, Health);
{% endhighlight %}

Unfortunately Unity is still stuck on .NET 3.5 which may soon change with IL2CPP, but for the time being you have to stick with .NET 3.5.

Unity's gameplay framework is very low level. You have complete freedom to do what and how you want. UE4 forces you to use it's gameplay framework which to be fair can fit many games genres but you have not much freedom in your architecture. A lot of stuff in UE4's gameplay framwork uses inheritance which makes it very rigid.

The good thing is that you are free to create you own stuff from scratch.

In Unity you have to use a GameObject and then you can attach MonoBehaviours to it. UE4 has something similar called Actors and ActorComponents. The main difference is that you can subclass an Actor. For example you could create 

{% highlight c++ %}
  class AMonster: public AActor
{% endhighlight %}

And at runtime you can check if some actor is a monster

{% highlight c++ %}
  AMonster* Monster = Cast<AMonster>(SomeActor);
{% endhighlight %}

While in Unity you would probably create a monster component like this

{% highlight c# %}
public class Monster: MonoBehevaior
{% endhighlight %}


{% highlight c# %}
Monster monster = gameObject.GetComponent<Monster>();
{% endhighlight %}

Alternatively you could use tags 
{% highlight c# %}
gameObject.tag = "Monster";
{% endhighlight %}

But I heavily dislike to use strings this way.

#Examples
Unity as well as UE4 have an insane amount of learning content. Unfortunately most if it is how to use the engine in general. There are almost no advanced coding examples at all.

A big plus point for UE4 is that Epic Games 'openly' develops Unreal Tournament which you can use to learn how Epic uses the engine internally.

#Networking
UE4s networking workflow is super awesome. You can just hit play and the editor will spawn multiple game instances and connect them automatically. Unity doesn't really have a workflow for multiplayer games. The main advantage of C# for me is the compilation speed but in Unity you have to build a standalone game if you want to have multiple game instances and building those binaries takes some time.

That is the reason why I have wrote my self a super hacky editor script that spawns 4 game instances, puts them on my second monitor, tiles them perfectly, starts the editor as a server and connects the 4 clients to it.

You can find the script here [UNetHelpers](https://github.com/MaikKlein/UNetHelpers)

In general UE4's networking and Unity's UNet feel very similar, but UNet comes with additional features. Unity has a matchmaking service, nat punchthough, a lobby system and it is easy to create a master server. Unity can also host your game on the AWS cloud for a small fee and it is even free for a small number of users which makes it very attractive to an 'indie' developer like me. 

I can host my prototype game right now and play with my friends. In UE4 this is a bit more tricky, there is no nat punchthough so you have to open some ports. But at least in UE4 you have the option to build a dedicated server, if you are willing to build from source.

In Unity you have the possibility to send messages on different channel. For example you can send position updates on channel 0 and more important messages on channel 1. You can also only set the message protocol on a specific channel. For example you can set channel 0 to unreliable and channel 1 to reliable. 

UE4 in contrast marks individual properties as reliable/unreliable and creates a channel for every actor that you create. It then uses [FObjectReplicator](https://docs.unrealengine.com/latest/INT/API/Runtime/Engine/Net/FObjectReplicator/index.html) to replicate the properties.

You can then control the priority of an actor with [GetNetPriority](https://docs.unrealengine.com/latest/INT/API/Runtime/Engine/GameFramework/AActor/GetNetPriority/2/index.html).

#Pricing 
UE4 comes without any restrictions but wants a cut from your revenue. Unity ships with two different version, Personal and Professional. The Personal edition has almost no restrictions besides that you have to earn less than $100k a year, a non customizable splash screen and no dark editor ui. The Professional includes a bunch of services. You can read more about it [here](https://unity3d.com/get-unity).

What I find really annoying is that you can not customize the splash screen in Unity Personal. I don't really care if my users can see the Unity splash screen, what I care about is that I have to build a standalone game to develop with Unity UNet. That means I have two watch the splash screen every time I want to test something. Unity should allow you to disable the splash screen in dev builds.

#Community
This comes up a lot but I don't really think it matters that much. If you are a lone developer like me you will rarely find help in forums even if you isolate your problems very well.

Unity has probably a much bigger community than UE4. If you would compare the numbers of the two reddit's then Unity's community would be 2.7 times bigger.

[UE4](https://www.reddit.com/r/unrealengine) / [Unity](https://www.reddit.com/r/unity3d)

Still a huge community is always a good thing especially if you run into some common bugs.

