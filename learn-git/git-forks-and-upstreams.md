# Git Fork and Upstreams ：如何去做一个很酷的技巧

Fork 项目来进行自己的修改，可以让你轻松地整合自己的贡献，但是如果你没有将这些修改发回上游-也就是发回父亲仓库--你就有可能失去对他们的跟踪，这可能会导致你的版本库中出现不同的线路。为了确保所有贡献者都从同有个地方获取信息，你需要了解一些关于git forking 与git upstream 如何交互的原理。在这篇博客中，我将向你介绍基础知识，疑难杂症，甚至给你留下一个很酷的小技巧，让你走在曲线的前面。



## Git Upstream: 保持最新做出贡献

让我先详细介绍一下与上游仓库交互的常见设置和最基本的工作流程。



在一个标准的设置中，你通常有一个origin和一个upstream remote -后者是项目的守门人，或者你希望贡献的真实来源。

首先，缺人你已经为上游仓库设置了一个remote, 并希望也设置了一个origin:

```
$ git remote -v 
origin git@bitbucket.org:my-user/some-project.git (fetch) 
origin git@bitbucket.org:my-user/some-project.git (push)
```

如果你没有上游，你可以很容易地用远程命令添加它。

```shell
git remote add upstream git@bitbucket.org:some-gatekeeper-maintainer/some-project.git
```

检查remote添加成功:

```shell
git remote -v 
origin git@bitbucket.org:my-user/some-project.git (fetch) 
origin git@bitbucket.org:my-user/some-project.git (push) 
upstream git@bitbucket.org:some-gatekeeper-maintainer/some-project.git (fetch) 
upstream git@bitbucket.org:some-gatekeeper-maintainer/some-project.git (push)
```

现在你可以用fetch收集上游仓库的最新变化。每次想要获得更新时，都要重复这个动作。

（如果项目的标签害没有合并到master,你还应该这样做：git fetch upstream --tags)

```shell
git fetch upstream
```

一般来说，你要保持本地分支作为上游主分支的近似镜像，并在特性分支中执行任何工作，因为它们以后可能会成为pull request .

在这一点，使用merge还是rebase并不重要，因为结果通常是一样的。让我们使用merge：

```shell
git checkout master
git merge upstream/master
```

当你想合上游的维护者分享一些工作时，你可以从master分支中创建一个特性分支，当你满意时，把它推送到你的远程仓库。

你也可以用rebase来代替，然后合并以确保上游有一套干净的提交（最好是一个）来评估。

```shell
git checkout -b feature-x 
#some work and some commits happen #some time passes git fetch upstream git rebase upstream/master
```

如果你需要把几个提交压成一个，你可以在这时使用厉害的rebase互动。

## Publish with git fork

经过以上步骤后，通过简单的push, 在远程fork中发布你的作品。

```shell
git push origin feature-x
```

如果你在发布远程分支feature-x后，因为上游维护者的一些反馈而不得不更新它，就会出现一个小问题，你有几个选择：

- 创建一个新的分支，包含你和上游的更新
- 将上游的更新合并到你的本地分支，并记录一个合并提交，这将使上游版本库混乱
- 在上游跟新的基础上重新建立本地分支，然后强制推动到远程分支

```shell
git push -f origin feature-x
```

我个人更倾向于尽量保持历史记录的干净，选择方案三，但不同的团队有不同的工作流程。注意：只有在使用你自己的fork时，你才能这样做，重写共享库和分支的历史是绝对不应该做的。重写共享库和分支的历史是你永远不应该改做的事情。

## Tip of the day: Ahead/Behind number in the promt 

fetch 之后，git status 会显示你比同步的远程分支提前或落后多少次提交。如果你能在你忠实的命令行提示符下看到这些信息，岂不是更好？我也是这么想的，所以我开始用我的bash chopsticks 并制作了它。

下面是你配置好后，它在你的提示符上的样子：

```shell
nick-macbook-air:~/dev/projects/stash[1|94]$
```

这就是你需要添加到你的.bashrc或等价物中的东西，只是一个函数：

```shell
function ahead_behind { 
  curr_branch=$(git rev-parse --abbrev-ref HEAD);
  curr_remote=$(git config branch.$curr_branch.remote);
  curr_merge_branch=$(git config branch.$curr_branch.merge | cut -d / -f 3);
  git rev-list --left-right --count $curr_branch...$curr_remote/$curr_merge_branch | tr -s '\t' '|'; 
}
```

你可以用这个新函数ahead_behind来丰富你的bash提示，已达到想要的效果。我把着色的工作留给读者去做。

Sampl prompt:

```shell
export PS1="\h:\w[\$(ahead_behind)]$"
```

### 内部结构

对于那些喜欢细节和解释的人来说，这是如何工作的。

我们得到当前HEAD的符号名称，及当前分支。

```shell
curr_branch=$(git rev-parse --abbrev-ref HEAD);
```

我们得到当前分支所指向的远程：

```shell
curr_remote=$(git config branch.$curr_branch.remote);
```

我们得到了这个远程分支应该被合并到的分支上（通过一个廉价的Unix技巧来丢弃包括最后一个斜杠[/]在内的所有内容）。

```shell
curr_merge_branch=$(git config branch.$curr_branch.merge | cut -d / -f 3);
```

现在我们有了我们需要的东西来手机我们领先和落后的提交次数

```shell
git rev-list --left-right --count $curr_branch...$curr_remote/$curr_merge_branch | tr -s '\t' '|';
```

我们使用古老的Unix tr将TAB转换为分隔符|.

## Getting started with git upstream

这就是git upstream 的基本演练，-- 如何设置git upstream, 创建新分支，收集修改，用git fork 发布，还有一个贴心的提示，这就是你的远程分支领先/落后于你多少次提交。

[原文链接](https://www.atlassian.com/git/tutorials/git-forks-and-upstreams)

