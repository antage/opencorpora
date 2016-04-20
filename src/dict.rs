use std::rc::Rc;

/// Структура описывающая граммему.
///
/// Список всех граммем можно [посмотреть](http://opencorpora.org/dict.php?act=gram)
/// на сайте [opencorpora.org](http://opencorpora.org).
#[derive(Clone, Default)]
pub struct Grammeme {
    /// Имя родительской граммемы (значение `None` индицирует, что это граммема верхнего уровня)
    pub parent: Option<String>,

    /// Имя граммемы на латинице
    pub name: String,

    /// Имя граммемы на кириллице
    pub alias: String,

    /// Подробное описание граммемы
    pub description: String,
}

/// Тип ограничения на использование граммемы.
#[derive(Clone, Copy)]
pub enum RestrictionKind {
    /// Необязательный
    Maybe,

    /// Обязательный
    Obligatory,

    /// Запрещающий
    Forbidden,
}

/// Область ограничения на использование граммемы.
#[derive(Clone, Copy)]
pub enum RestrictionScope {
    /// Лексема
    Lemma,

    /// Словоформа лексемы
    Form,
}

/// Правило ограничивающие применение граммемы.
///
/// Более подробное описание [приведено](http://opencorpora.org/dict.php?act=gram_restr)
/// на сайте [opencorpora.org](http://opencorpora.org).
#[derive(Clone)]
pub struct Restriction {
    /// Тип ограничения (см. [документацию](enum.RestrictionKind.html) типа `RestrictionKind`)
    pub kind: RestrictionKind,

    /// Приоритет (?)
    pub auto: usize,

    /// Область применения слева
    pub left_scope: RestrictionScope,

    /// Граммема слева.
    /// Для некоторых правил значение справа может отсутствовать.
    pub left_grammeme: Option<Rc<Grammeme>>,

    /// Область применения справа
    pub right_scope: RestrictionScope,

    /// Граммема справа.
    /// Для некоторых правил значение справа может отсутствовать.
    pub right_grammeme: Option<Rc<Grammeme>>,
}

impl Default for Restriction {
    fn default() -> Self {
        Restriction{
            kind: RestrictionKind::Maybe,
            auto: 0,
            left_scope: RestrictionScope::Lemma,
            left_grammeme: None,
            right_scope: RestrictionScope::Lemma,
            right_grammeme: None,
        }
    }
}

/// Структура словоформы лексемы.
#[derive(Clone, Default)]
pub struct Form {
    /// Текстовое представление словоформы
    pub word: String,

    /// Множество граммем описывающих словоформу
    pub grammemes: Vec<Rc<Grammeme>>,
}

/// Структура описывающая лексему.
#[derive(Clone, Default)]
pub struct Lemma {
    /// Числовой идентификатор лексемы
    pub id: usize,

    /// Номер ревизии
    pub revision: usize,

    /// Текстовое представление исходный словоформы лексемы
    pub word: String,

    /// Множество граммем описывающих лексему
    pub grammemes: Vec<Rc<Grammeme>>,

    /// Множество словоформ входящих в данную лексему
    pub forms: Vec<Form>,
}

/// Тип связи между лексемами.
#[derive(Clone, Default)]
pub struct LinkKind {
    /// Числовой идентификатор типа связи.
    /// Используется в типе `Link`.
    pub id: usize,

    /// Имя типа связи
    pub name: String,
}

/// Структура хранящая связь между двумя лексемами.
#[derive(Clone, Default)]
pub struct Link {
    /// Числовой идентификатор связи
    pub id: usize,

    /// Лексема с исходной стороны связи
    pub from: Rc<Lemma>,

    /// Лексема с конечной стороны связи
    pub to: Rc<Lemma>,

    /// Типа связи
    pub kind: Rc<LinkKind>,
}

/// Структура содержащая данные словаря.
#[derive(Default)]
pub struct Dict {
    /// Версия словаря
    pub version: String,

    /// Номер ревизии
    pub revision: usize,

    /// Множество граммем
    pub grammemes: Vec<Rc<Grammeme>>,

    /// Множество правил-ограничений на использование граммем
    pub restrictions: Vec<Restriction>,

    /// Массив лексем
    pub lemmata: Vec<Rc<Lemma>>,

    /// Множество типов связей между лексемами
    pub link_kinds: Vec<Rc<LinkKind>>,

    /// Множество связей между лексемами
    pub links: Vec<Link>,
}
