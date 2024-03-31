---
marp: true
# theme: 
# class: 
style: |
    * {
        font-family: Times New Roman;
    }
    footer {
        border-top: 1px solid #999;
        font-size: small;
        opacity: 0.8; 
    }
    header {
        margin-top: 0px;
        border-bottom: 1px solid #999;
        font-size: 66px;
    }
    .head {
        font-size: 26px
    }
    .columns {
        display: grid;
        grid-template-columns: repeat(2, minmax(0, 1fr));
        gap: 1rem;
    }
    .left{
        text-align: left 
    }
---

<div class="head">
 МОСКОВСКИЙ ГОСУДАРСТВЕННЫЙ ТЕХНИЧЕСКИЙ УНИВЕРСИТЕТ ИМЕНИ Н.Э. БАУМАНА (НАЦИОНАЛЬНЫЙ ИССЛЕДОВАТЕЛЬСКИЙ УНИВЕРСИТЕТ)
</div>

# «Анализ методов распределенных вычислений в распределенных системах хранения информации» 


<div class="left">
<b>Студент:</b> Романов Семен Константинович

<b>Группа:</b> ИУ7-75Б
<b>Научный руководитель:</b> Бекасов Денис Евгеньевич

</div>


---
<style scoped>
    .left {
        margin-top: 0px;
        font-size: 40px
    }
    li {
        font-size: 34px;
    }
</style>
    
# <header>Цель и задачи
</header>

<div class="left"><b>Цель</b> – классифицировать существующие методы распределенных вычислений.</div>

##### <div class="left">Задачи:
* Провести обзор существующих систем распределенных вычислений;
* Провести анализ подходов к проектированию распределенных вычислений;
* Cформулировать критерии сравнения методов распределенных вычислений;
</div>

--- 

<style scoped>
    .marg {
        margin-top: 0px;
        font-size: 60px;
    }
    li {
        font-size: 30px;
    }
    
</style>

# <header> Особенности распределенных систем
</header>

## <div class="marg"> Особенности распределенных систем: </div>

<ul style="list-style-type: '— '">
    
<li> Распределенная система — это вычислительная среда, в которой многочисленные компоненты расположены на нескольких вычислительных устройствах в сети.
<li> Мотивацией роста распределенных вычислений является доступность недорогих, высокопроизводительных компьютеров и сетевых инструментов.
<li> Распеделенная система может обладать более высокой производительностью, чем один конкретный суперкомпьютер
<li> Основным компонентом во всех архитектурах распределенных вычислений является понятие связи между узлами системы.
<ul>


---

<style scoped>
    header {
        margin-top: 0px;
        font-size: 50px;
    }
    .left {
        margin-top: 100px;
        font-size: 34px;
    }
    li {
        font-size: 30px;
    }
    img[alt~="center"] {
    margin-top: 150px;
    display: block;
}
</style>

# <header> Сравнение производительности различных вычислительных систем.
</header>

![fit center](../src/inc/img/dist.png)

---
<style scoped>
    header {
        margin-top: 0px;
        font-size: 60px;
    }
    .marg {
        margin-top: 0px;
        font-size: 60px;
    }
    .left {
        font-size: 32px;
    }
</style>

# <header>Базовые понятия
</header>

## <div class="marg">Обработка данных</div>
<div>
    Пакетная обработка
    <ul style="list-style-type: '— '">
        <li> Одновременная обработка нескольких обращений.
        <li> Разделяется на следующие типы:
        <ol type="1">
        <li> Одновременная пакетная обработка (объекты типа "B")
        <li> Последовательная пакетная обработка (объекты типа "A")
        <li> Параллельная пакетная обработка (объекты типа "C", "D", "E", "F" и "G")
        </ol>
    </lu>
</div>

![h:140](../src/inc/img/batch.png)

---
<style scoped>
    header {
        margin-top: 0px;
        font-size: 60px;
    }
    .marg {
        margin-top: 0px;
        font-size: 60px;
    }
    .left {
        font-size: 32px;
    }
</style>


# <header>Базовые понятия
</header>

## <div class="marg">Обработка данных</div>
<div>
    Потоковая обработка
    <ul style="list-style-type: '— '">
        <li> Анализ элементов из потоков данных по мере их поступления.
        <li> Включает в себя следующий набор ключевых понятий:
        <ol type="1">
        <li> Модель потока данных – определяет структуру и алгоритм поступления данных в систему
        <li> Представление времени – необходимый элемент системы, необходимый для синхронизации данных
        <li> Окна –  используются для выполнения вычислений, которые были бы невозможны (бесконечны) в случае неограниченных данных
        </ol>
    </lu>
</div>

---

<style scoped>
    header {
        margin-top: 0px;
        font-size: 60px;
    }
    .marg {
        /* margin-top: 80px; */
        font-size: 60px;
    }
    .left {
        font-size: 32px;
    }
</style>

# <header>Существующие решения: Hadoop MapReduce
</header>

## <div class="marg">Программная модель</div>

<ul style="list-style-type: '— '">
    <li> Вычисления в MapReduce используют набор входных пар ключ/значение и создает набор выходных пар ключ/значение.
    <li> Функция Map принимает входную пару и создает набор промежуточных пар ключ/значение
    <li> Функция Reduce принимает промежуточный ключ I и набор значений для этого ключа, после чего объединяет эти значения для формирования возможно меньшего набора значений
</lu>

---
<style scoped>
    header {
        margin-top: 0px;
        font-size: 60px;
    }
    .marg {
        margin-top: 0px;
        font-size: 60px;
    }
    .left {
        font-size: 32px;
    }
    img[alt~="center"] {
  display: block;
  margin: 0 auto;
}
</style>

# <header>Существующие решения: Hadoop MapReduce
</header>

## <div class="marg">Реализация</div>

![w:800 center](../src/inc/img/map_reduce.png)

---
<style scoped>
    header {
        margin-top: 0px;
        font-size: 60px;
    }
    .marg {
        margin-top: 0px;
        font-size: 60px;
    }
    .left {
        font-size: 32px;
    }
    img[alt~="center"] {
  display: block;
  margin: 0 auto;
}
</style>

# <header>Hadoop MapReduce
</header>
Включает в себя следующие компоненты:
<lu style="list-style-type: '— '">
    <li> HBase
    <li> Hive
    <li> HDFS
    <li> Sqoop
    <li> Solr
    <li> Flume
    <li> YARN
    <li> ZooKeeper
    <li> Pig
</ul>

![bg fit right](../src/inc/img/hadoop-eco.png)

---
<style scoped>
    header {
        margin-top: 0px;
        font-size: 60px;
    }
    .marg {
        /* margin-top: 80px; */
        font-size: 60px;
    }
    .left {
        font-size: 32px;
    }
</style>

# <header>Существующие решения: Spark
</header>

## <div class="marg">Программная модель</div>

<ul style="list-style-type: '— '">
    <li> Основной абстракцией в Spark является устойчивый распределенный набор данных (Resilient Distributed Dataset – RDD).
    <li> RDD — это доступная только для чтения секционированная коллекция записей. 
    RDD могут быть созданы только с помощью детерминированных операций либо с данными в некотором стабильном хранилище, либо с другими RDD.
    <li> Вторая важная абстракция – параллельные операции с этими наборами данных, вызываемые путем передачи функции для применения к набору данных.
    <li> Операции разделяются на два типа: преобразования – отложенные (ленивые) операции, и действия, которые запускают вычисление для возврата значения программе или записи данных во внешнее хранилище
</lu>

---
<style scoped>
    header {
        margin-top: 0px;
        font-size: 60px;
    }
    .marg {
        margin-top: 120px;
    }
    .left {
        font-size: 32px;
    }
    img[alt~="center"] {
  display: block;
  margin: 0 auto;
}
</style>

# <header>Существующие решения: Spark
</header>

<div class="marg">
Всякий раз, когда пользователь запускает действие в RDD, планировщик проверяет линейный граф этого RDD, чтобы построить список этапов для выполнения. Каждый этап как правило содержит наибольшее возможное количество конвейерных преобразований с узкими зависимостями, т.е. теми, где каждый раздел родительского RDD используется не более чем одним разделом дочернего RDD.
</div>

![bg fit right](../src/inc/img/spark_stages.png)


---
<style scoped>
    header {
        margin-top: 0px;
        font-size: 60px;
    }
    .marg {
        margin-top: 30px;
        font-size: 60px;
    }
    .left {
        font-size: 32px;
    }
</style>

# <header>Существующие решения: Pregel
</header>

## <div class="marg">Программная модель</div>

<ul style="list-style-type: '— '">
    <li> Входными данными для вычислений в Pregel является ориентированный граф, в котором каждая вершина однозначно идентифицируется строковым идентификатором вершины.
    <li> Вычисления в Pregel состоят из последовательности итераций, называемых супершагами, разделенных точками глобальной синхронизации. Во время супершага фреймворк вызывает определенную пользователем функцию для каждой вершины, практически параллельно. 
    <li> Завершение алгоритма основано на том, что каждая вершина графа голосует за остановку. Вершина деактивирует себя, проголосовав за остановку алгоритма. Алгоритм
завершается, когда все вершины одновременно неактивны и нет передаваемых
сообщений.
</lu>

---
<style scoped>
    header {
        margin-top: 0px;
        font-size: 60px;
    }
    .marg {
        margin-top: 120px;
    }
    .left {
        font-size: 32px;
    }
</style>

# <header>Существующие решения: Pregel </header>

<div class="marg">
Pregel делит граф на разделы, каждый из которых состоит из набора вершин и всех исходящих ребер этих вершин.
На каждом шаге каждая вершина считывает все входящие сообщения и устанавливает максимальное значение для своего текущего значения. Затем он отправляет это максимальное значение по всем своим ребрам. Если значение в узле не изменяется во время супершага, узел голосует за остановку.
</div>

![bg h:450 right](../src/inc/img/pregel_impl.png)

---
<style scoped>
    header {
        margin-top: 0px;
        font-size: 60px;
    }
    .marg {
        margin-top: 30px;
        font-size: 60px;
    }
    .left {
        font-size: 32px;
    }
</style>

# <header>Существующие решения: DryadLINQ
</header>

## <div class="marg">Программная модель</div>

<ul style="list-style-type: '— '">
    <li> Термин LINQ относится к набору конструкций .NET для манипулирования наборами и последовательностями элементов данных.
    <li> Dryad в свою очередь выступает как механизм распределенного выполнения LINQ выражений.
    <li> DryadLINQ сохраняет модель программирования LINQ и расширяет ее для параллельного программирования, определяя небольшой набор новых операторов и типов данных.
    <li> Все функции, вызываемые в выражениях DryadLINQ, должны быть свободны от побочных эффектов.
</lu>

---
<style scoped>
    header {
        margin-top: 0px;
        font-size: 60px;
    }
    .marg {
        margin-top: 120px;
    }
    .left {
        font-size: 32px;
    }
</style>

# <header>Существующие решения: DryadLINQ </header>

<div class="marg">
Когда DryadLINQ получает управление, система начинает с преобразования необработанного выражения LINQ в граф плана выполнения (EPG), где каждый узел является оператором, а ребра представляют его входы и выходы. 
EPG используется для получения плана выполнения Dryad. DryadLINQ использует динамическую генерацию кода для автоматического синтеза кода LINQ для запуска в вершинах Dryad.
</div>

![bg h:400 right](../src/inc/img/dryad_arch.png)

---
<style scoped>
    header {
        margin-top: 0px;
        font-size: 60px;
    }
    .marg {
        margin-top: 30px;
        font-size: 60px;
    }
    .left {
        font-size: 32px;
    }
</style>

# <header>Существующие решения: Google Dataflow 
</header>

## <div class="marg">Программная модель</div>

<ul style="list-style-type: '— '">
    <li> Модель Dataflow содержит две основных операций, которые работают с парами ключ-значение, проходящими через систему: ParDo для универсальной параллельной обработки и GroupByKey для группируемых пар ключ-значение по ключу.
    <li> Операция ParDo работает поэлементно с каждым входным элементом и, таким образом преобразуется в работу с потоковыми данными. 
    <li> Операция GroupByKey, с другой стороны, собирает все данные для данного ключа перед отправкой их вниз по потоку.
    <li> В случае если источник входных данных неограничен, то используется обработка данных окнами.
</lu>

---
<style scoped>
    header {
        margin-top: 0px;
        font-size: 60px;
    }
    .marg {
        margin-top: 00px;
        font-size: 60px;
    }
    .left {
        font-size: 32px;
    }
</style>

# <header>Существующие решения: Apache Flink
</header>

## <div class="marg">Программная модель</div>

![fit center](../src/inc/img/flink_model.png)

---
<style scoped>
    header {
        margin-top: 0px;
        font-size: 60px;
    }
    .marg {
        margin-top: 120px;
    }
    .left {
        font-size: 32px;
    }
    img[alt~="center"] {
    margin-top: 50px;
    display: block;
}
</style>

# <header>Существующие решения: Apache Samza </header>

![center](../src/inc/img/flink_arch.png)

---
<style scoped>
    header {
        margin-top: 0px;
        font-size: 60px;
    }
    .marg {
        margin-top: 200px;
    }
    .left {
        font-size: 32px;
    }
    img[alt~="center"] {
    margin: 0 auto;
    display: block;
}
</style>

# <header>Существующие решения: Apache Samza </header>

<div class="marg">
Задача в Samza состоит из набора экземпляров виртуальной машины Java, каждый из которых обрабатывает подмножество входных данных. Samza представляет задачи в виде ориентированного графа операторов (вершин), соединенных потоками данных (ребрами). Оператор — это преобразование одного или многих потоков в другой по-
ток (потоки). 
<div>

![bg w:500 left](../src/inc/img/samza-job.png)

---
<style scoped>
    * {
        font-size: 22px;
    }
    header {
        margin-top: 0px;
        font-size: 48px;
    }
    .marg {
        margin-top: -40px;
        font-size: 40px;
    }
    .left {
        font-size: 32px;
    }
</style>

# <header>Классификация методов распределенных вычислений
</header>

## <div class="marg">Классификация моделей по сценариям использования</div>

Модель                |   Сценарий использования 
:-----                  |   :------
MapReduce      |   Пакетная обработка больших наборов данных
Apache Spark            |   Пакетная обработка данных в реальном времени и итеративные алгоритмы
Pregel     |   Обработка графов большого масштаба
DryadLinq                |   Распределенная обработка данных общего назначения, научные вычисления
Google Dataflow        |   Непрерывная обработка данных в реальном времени, приложения, управляемые событиями
Apache Flink               |   Обработка потоков данных, приложения, управляемые событиями, аналитика в реальном времени
Apache Samza  |   Обработка потоков данных, приложения, управляемые сообщениями

---
<style scoped>
    * {
        font-size: 30px;
    }
    header {
        margin-top: 0px;
        font-size: 68px;
    }
    .marg {
        margin-top: 0px;
        font-size: 50px;
        text-align: left;
    }
    .left {
        margin-top: 60px;
        font-size: 32px;
    } 
</style>

# <header>Выводы
</header>

## <div class="marg">В результате данной работы:</div>

<ul style="list-style-type: '— '">
<li> был проведен обзор существующих систем распределенных вычислений;
<li> был проведен анализ подходов к проектированию распределенных вычислений;
<li> были cформулированы критерии сравнения методов распределенных вычислений.
<lu>

<div class="left">
Цель – классифицировать существующие методы распределенных вычислений – была выполнена.
</div>
