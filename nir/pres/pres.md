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

# <header>Существующие решения: Hadoop MapReduce
</header>
Включает в себя следующие компоненты:
<lu style="list-style-type: '— '">
    <li> HBase
    <li>
    <li>
    <li>
    <li>
</ul>

![bg fit right](../src/inc/img/hadoop-eco.png)


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
        margin-top: 20px;
        font-size: 40px;
    }
    .left {
        font-size: 32px;
    }
</style>

# <header>Классификация методов модификации ядра Linux
</header>

## <div class="marg">Критерии сравнения методов модификации ядра</div>


Критерий                |   Описание 
:-----                  |   :------
Производительность      |   Производительность программ
Безопасность            |   Наличие гарантии, что внесенный код не вызовет остановку системы
Скорость разработки     |   Является ли метод быстрым в разработке
Гибкость                |   Возможность метода подстроиться под любые поставленные задачи
Простота отладки        |   Является ли описанная модификация простой в отладке
Поддержка               |   Поддержка метода разработчиками ядра при его написании
Простота развёртывания  |   Является ли описанный метод простым в развёртывании на большом количестве машин

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
        margin-top: 20px;
        font-size: 40px;
        margin-bottom: 20px;
    }
    .left {
        font-size: 32px;
    }
</style>

# <header>Классификация методов модификации ядра Linux
</header>

## <div class="marg">Критерии сравнения методов модификации ядра</div>


Критерий                |   Рекомпиляция    |   LKM     |   Live Patching   |   eBPF
:-----                  |   :------:        |   :------:|   :------:        |   :------:
Производительность      | :white_check_mark:  | :white_check_mark: | :white_check_mark: | :white_check_mark:
Безопасность            | :x:  | :x: | :x: | :white_check_mark: 
Скорость разработки     | :x:  | :white_check_mark: | :x: | :white_check_mark:
Гибкость                | :white_check_mark:  | :white_check_mark: | :x: | :x:
Простота отладки        | :x:  | :white_check_mark:/:x: | :x: | :white_check_mark:
Поддержка               | :white_check_mark:  | :white_check_mark: | :white_check_mark: | :x:
Простота развёртывания  | :x:  | :white_check_mark: | :white_check_mark: | :white_check_mark:

---
<style scoped>
    * {
        font-size: 30px;
    }
    header {
        margin-top: 0px;
        font-size: 48px;
    }
    .marg {
        margin-top: 20px;
        font-size: 40px;
        text-align: left;
    }
    /* .left {
        font-size: 32px;
    } */
</style>

# <header>Выводы
</header>

## <div class="marg">В ходе данной работы были изучены:</div>

- методы модификации ядра Linux;
- критерии сравнения методов модификации ядра;
- основные принципы работы и преимущества каждого из методов.

<div class="left">
Был выполнен обзор существующих методов модификации ядра Linux, проведен анализ их преимуществ и недостатков.
</div>

<div class="left">
Были сформулированы критерии классификации методов модификации ядра Linux.
Была проведена классификация методов модификации ядра Linux по критериям, сформулированным в ходе работы.
</div>