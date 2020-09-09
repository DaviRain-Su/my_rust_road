# rwiser中的文本提取和存储

## 用于处理XML的2中API- DOM 和 SAX

背景：

由于所以使用的Wikipedia的词条数据全部来源于1个巨大的XML文件，所以想要通过wiser构建索引，就需要包含了所有词条的XML文件中提取出各个词条（文档），然后再从各个词条中提取出标题和正文。

为了加载XML文件，我们在wiser中使用了名为expat的代码库。用于处理XML的API分为以下两种；
- DOM (Document Object Model)
- SAX (Simple API for XML)

使用DOM时，会先将整个XML文档全部加载到内存中，然后再开始解析处理。也就是说，这种方法的
优点在于操作时间可以忽略元素的顺序，而缺点在于会消耗大量的内存。虽然DOM使用起来很方便，但是会占用大量的资源。与此相反，在使用SAX时，由于是一遍加载XML中的元素，一边一次进行处理的，所以只需少量的内存即可完成处理，但是处理时不得不考虑XML文档中元素的顺序。综上所述，DOM和SAX各有各的优点和缺点。

由于包含的Wikipedia词条的XML文件相对来说比较大。所以对于现在的个人计算机而言，要把所有的数据都加载都内存上恐怕有些困难，因此，在wiser中，我们会使用SAX来处理XML。

## 提取文档的标题和正文

在wiser中，有关Wikipdia词条数据中提取文档的处理过程都写在了wikiload.c中的函数load_wikipedai_dump()中。由于该函数中并没有进行十分复杂处理，所以我们就一行一行的往下读吧。


简单来说，函数load_wikipedia_dump()的作用是用SAX解析含有Wikipedia词条的XML文档，并从中提取出词条的标题和正文。（目标是提取词条的标题和正文）提取出的标题和正文会通过
存储文档的回调函数存储到数据库中，而该回调函数会作为参数传入函数load_wikipedia_dump()中。

下面开始分析load_wikipedia_dump() 的源代码

```
/**
 * 加载Wikipedia的副本（XML文件），并将其内容传递给指定的函数
 * @param[in] env 存储着应用程序运行环境的结构体
 * @param[in] path Wikipedia副本的路径
 * @param[in] func 接收env，词条标题，词条正文3个参数的回调函数（参看wiser.c的223行）
 * @param[in] max_article_count 最多加载多少个词条
 * @retval 0 成功
 * @retval 1 申请内存失败
 * @retval 2 打开文件失败
 * @retval 3 加载文件失败
 * @retval 4 解析XML文件失败
 */
int
load_wikipedia_dump(wiser_env *env,
                    const char *path, add_document_callback func, int max_article_count)
{
  FILE *fp;
  int rc = 0;
  XML_Parser xp;
  char buffer[LOAD_BUFFER_SIZE];
  wikipedia_parser wp = {
    env,               /* 存储着应用程序运行环境的结构体 */
    IN_DOCUMENT,       /* 初始状态 */
    NULL,              /* 词条标题的临时存储区 */
    NULL,              /* 词条正文的临时存储区 */
    0,                 /* 初始化经过解析的词条总数 */
    max_article_count, /* 最多要解析多少个词条 */
    func               /* 将解析后的文档传递给该函数 */
  };

  if (!(xp = XML_ParserCreate("UTF-8"))) {
    print_error("cannot allocate memory for parser.");
    return 1;
  }

  if (!(fp = fopen(path, "rb"))) {
    print_error("cannot open wikipedia dump xml file(%s).",
                strerror(errno));
    rc = 2;
    goto exit;
  }

  XML_SetElementHandler(xp, start, end);
  XML_SetCharacterDataHandler(xp, element_data);
  XML_SetUserData(xp, (void *)&wp);

  while (1) {
    int buffer_len, done;

    buffer_len = (int)fread(buffer, 1, LOAD_BUFFER_SIZE, fp);
    if (ferror(fp)) {
      print_error("wikipedia dump xml file read error.");
      rc = 3;
      goto exit;
    }
    done = feof(fp);

    if (XML_Parse(xp, buffer, buffer_len, done) == XML_STATUS_ERROR) {
      print_error("wikipedia dump xml file parse error.");
      rc = 4;
      goto exit;
    }

    if (done || (max_article_count >= 0 &&
                 max_article_count <= wp.article_count)) { break; }
  }
exit:
  if (fp) {
    fclose(fp);
  }
  if (wp.title) {
    utstring_free(wp.title);
  }
  if (wp.body) {
    utstring_free(wp.body);
  }
  XML_ParserFree(xp);
  return rc;
}

```