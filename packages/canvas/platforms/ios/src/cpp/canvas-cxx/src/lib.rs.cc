#include "OnRafCallback.h"
#include <algorithm>
#include <array>
#include <cassert>
#include <cstddef>
#include <cstdint>
#include <initializer_list>
#include <iterator>
#include <new>
#include <stdexcept>
#include <string>
#include <type_traits>
#include <utility>
#if defined(_WIN32)
#include <basetsd.h>
#else
#include <sys/types.h>
#endif

namespace rust {
inline namespace cxxbridge1 {
// #include "rust/cxx.h"

#ifndef CXXBRIDGE1_PANIC
#define CXXBRIDGE1_PANIC
template <typename Exception>
void panic [[noreturn]] (const char *msg);
#endif // CXXBRIDGE1_PANIC

struct unsafe_bitcopy_t;

namespace {
template <typename T>
class impl;
} // namespace

template <typename T>
::std::size_t size_of();
template <typename T>
::std::size_t align_of();

#ifndef CXXBRIDGE1_RUST_STRING
#define CXXBRIDGE1_RUST_STRING
class String final {
public:
  String() noexcept;
  String(const String &) noexcept;
  String(String &&) noexcept;
  ~String() noexcept;

  String(const std::string &);
  String(const char *);
  String(const char *, std::size_t);
  String(const char16_t *);
  String(const char16_t *, std::size_t);

  static String lossy(const std::string &) noexcept;
  static String lossy(const char *) noexcept;
  static String lossy(const char *, std::size_t) noexcept;
  static String lossy(const char16_t *) noexcept;
  static String lossy(const char16_t *, std::size_t) noexcept;

  String &operator=(const String &) &noexcept;
  String &operator=(String &&) &noexcept;

  explicit operator std::string() const;

  const char *data() const noexcept;
  std::size_t size() const noexcept;
  std::size_t length() const noexcept;
  bool empty() const noexcept;

  const char *c_str() noexcept;

  std::size_t capacity() const noexcept;
  void reserve(size_t new_cap) noexcept;

  using iterator = char *;
  iterator begin() noexcept;
  iterator end() noexcept;

  using const_iterator = const char *;
  const_iterator begin() const noexcept;
  const_iterator end() const noexcept;
  const_iterator cbegin() const noexcept;
  const_iterator cend() const noexcept;

  bool operator==(const String &) const noexcept;
  bool operator!=(const String &) const noexcept;
  bool operator<(const String &) const noexcept;
  bool operator<=(const String &) const noexcept;
  bool operator>(const String &) const noexcept;
  bool operator>=(const String &) const noexcept;

  void swap(String &) noexcept;

  String(unsafe_bitcopy_t, const String &) noexcept;

private:
  struct lossy_t;
  String(lossy_t, const char *, std::size_t) noexcept;
  String(lossy_t, const char16_t *, std::size_t) noexcept;
  friend void swap(String &lhs, String &rhs) noexcept { lhs.swap(rhs); }

  std::array<std::uintptr_t, 3> repr;
};
#endif // CXXBRIDGE1_RUST_STRING

#ifndef CXXBRIDGE1_RUST_STR
#define CXXBRIDGE1_RUST_STR
class Str final {
public:
  Str() noexcept;
  Str(const String &) noexcept;
  Str(const std::string &);
  Str(const char *);
  Str(const char *, std::size_t);

  Str &operator=(const Str &) &noexcept = default;

  explicit operator std::string() const;

  const char *data() const noexcept;
  std::size_t size() const noexcept;
  std::size_t length() const noexcept;
  bool empty() const noexcept;

  Str(const Str &) noexcept = default;
  ~Str() noexcept = default;

  using iterator = const char *;
  using const_iterator = const char *;
  const_iterator begin() const noexcept;
  const_iterator end() const noexcept;
  const_iterator cbegin() const noexcept;
  const_iterator cend() const noexcept;

  bool operator==(const Str &) const noexcept;
  bool operator!=(const Str &) const noexcept;
  bool operator<(const Str &) const noexcept;
  bool operator<=(const Str &) const noexcept;
  bool operator>(const Str &) const noexcept;
  bool operator>=(const Str &) const noexcept;

  void swap(Str &) noexcept;

private:
  class uninit;
  Str(uninit) noexcept;
  friend impl<Str>;

  std::array<std::uintptr_t, 2> repr;
};
#endif // CXXBRIDGE1_RUST_STR

#ifndef CXXBRIDGE1_RUST_SLICE
#define CXXBRIDGE1_RUST_SLICE
namespace detail {
template <bool>
struct copy_assignable_if {};

template <>
struct copy_assignable_if<false> {
  copy_assignable_if() noexcept = default;
  copy_assignable_if(const copy_assignable_if &) noexcept = default;
  copy_assignable_if &operator=(const copy_assignable_if &) &noexcept = delete;
  copy_assignable_if &operator=(copy_assignable_if &&) &noexcept = default;
};
} // namespace detail

template <typename T>
class Slice final
    : private detail::copy_assignable_if<std::is_const<T>::value> {
public:
  using value_type = T;

  Slice() noexcept;
  Slice(T *, std::size_t count) noexcept;

  Slice &operator=(const Slice<T> &) &noexcept = default;
  Slice &operator=(Slice<T> &&) &noexcept = default;

  T *data() const noexcept;
  std::size_t size() const noexcept;
  std::size_t length() const noexcept;
  bool empty() const noexcept;

  T &operator[](std::size_t n) const noexcept;
  T &at(std::size_t n) const;
  T &front() const noexcept;
  T &back() const noexcept;

  Slice(const Slice<T> &) noexcept = default;
  ~Slice() noexcept = default;

  class iterator;
  iterator begin() const noexcept;
  iterator end() const noexcept;

  void swap(Slice &) noexcept;

private:
  class uninit;
  Slice(uninit) noexcept;
  friend impl<Slice>;
  friend void sliceInit(void *, const void *, std::size_t) noexcept;
  friend void *slicePtr(const void *) noexcept;
  friend std::size_t sliceLen(const void *) noexcept;

  std::array<std::uintptr_t, 2> repr;
};

template <typename T>
class Slice<T>::iterator final {
public:
  using iterator_category = std::random_access_iterator_tag;
  using value_type = T;
  using difference_type = std::ptrdiff_t;
  using pointer = typename std::add_pointer<T>::type;
  using reference = typename std::add_lvalue_reference<T>::type;

  reference operator*() const noexcept;
  pointer operator->() const noexcept;
  reference operator[](difference_type) const noexcept;

  iterator &operator++() noexcept;
  iterator operator++(int) noexcept;
  iterator &operator--() noexcept;
  iterator operator--(int) noexcept;

  iterator &operator+=(difference_type) noexcept;
  iterator &operator-=(difference_type) noexcept;
  iterator operator+(difference_type) const noexcept;
  iterator operator-(difference_type) const noexcept;
  difference_type operator-(const iterator &) const noexcept;

  bool operator==(const iterator &) const noexcept;
  bool operator!=(const iterator &) const noexcept;
  bool operator<(const iterator &) const noexcept;
  bool operator<=(const iterator &) const noexcept;
  bool operator>(const iterator &) const noexcept;
  bool operator>=(const iterator &) const noexcept;

private:
  friend class Slice;
  void *pos;
  std::size_t stride;
};

template <typename T>
Slice<T>::Slice() noexcept {
  sliceInit(this, reinterpret_cast<void *>(align_of<T>()), 0);
}

template <typename T>
Slice<T>::Slice(T *s, std::size_t count) noexcept {
  assert(s != nullptr || count == 0);
  sliceInit(this,
            s == nullptr && count == 0
                ? reinterpret_cast<void *>(align_of<T>())
                : const_cast<typename std::remove_const<T>::type *>(s),
            count);
}

template <typename T>
T *Slice<T>::data() const noexcept {
  return reinterpret_cast<T *>(slicePtr(this));
}

template <typename T>
std::size_t Slice<T>::size() const noexcept {
  return sliceLen(this);
}

template <typename T>
std::size_t Slice<T>::length() const noexcept {
  return this->size();
}

template <typename T>
bool Slice<T>::empty() const noexcept {
  return this->size() == 0;
}

template <typename T>
T &Slice<T>::operator[](std::size_t n) const noexcept {
  assert(n < this->size());
  auto ptr = static_cast<char *>(slicePtr(this)) + size_of<T>() * n;
  return *reinterpret_cast<T *>(ptr);
}

template <typename T>
T &Slice<T>::at(std::size_t n) const {
  if (n >= this->size()) {
    panic<std::out_of_range>("rust::Slice index out of range");
  }
  return (*this)[n];
}

template <typename T>
T &Slice<T>::front() const noexcept {
  assert(!this->empty());
  return (*this)[0];
}

template <typename T>
T &Slice<T>::back() const noexcept {
  assert(!this->empty());
  return (*this)[this->size() - 1];
}

template <typename T>
typename Slice<T>::iterator::reference
Slice<T>::iterator::operator*() const noexcept {
  return *static_cast<T *>(this->pos);
}

template <typename T>
typename Slice<T>::iterator::pointer
Slice<T>::iterator::operator->() const noexcept {
  return static_cast<T *>(this->pos);
}

template <typename T>
typename Slice<T>::iterator::reference Slice<T>::iterator::operator[](
    typename Slice<T>::iterator::difference_type n) const noexcept {
  auto ptr = static_cast<char *>(this->pos) + this->stride * n;
  return *reinterpret_cast<T *>(ptr);
}

template <typename T>
typename Slice<T>::iterator &Slice<T>::iterator::operator++() noexcept {
  this->pos = static_cast<char *>(this->pos) + this->stride;
  return *this;
}

template <typename T>
typename Slice<T>::iterator Slice<T>::iterator::operator++(int) noexcept {
  auto ret = iterator(*this);
  this->pos = static_cast<char *>(this->pos) + this->stride;
  return ret;
}

template <typename T>
typename Slice<T>::iterator &Slice<T>::iterator::operator--() noexcept {
  this->pos = static_cast<char *>(this->pos) - this->stride;
  return *this;
}

template <typename T>
typename Slice<T>::iterator Slice<T>::iterator::operator--(int) noexcept {
  auto ret = iterator(*this);
  this->pos = static_cast<char *>(this->pos) - this->stride;
  return ret;
}

template <typename T>
typename Slice<T>::iterator &Slice<T>::iterator::operator+=(
    typename Slice<T>::iterator::difference_type n) noexcept {
  this->pos = static_cast<char *>(this->pos) + this->stride * n;
  return *this;
}

template <typename T>
typename Slice<T>::iterator &Slice<T>::iterator::operator-=(
    typename Slice<T>::iterator::difference_type n) noexcept {
  this->pos = static_cast<char *>(this->pos) - this->stride * n;
  return *this;
}

template <typename T>
typename Slice<T>::iterator Slice<T>::iterator::operator+(
    typename Slice<T>::iterator::difference_type n) const noexcept {
  auto ret = iterator(*this);
  ret.pos = static_cast<char *>(this->pos) + this->stride * n;
  return ret;
}

template <typename T>
typename Slice<T>::iterator Slice<T>::iterator::operator-(
    typename Slice<T>::iterator::difference_type n) const noexcept {
  auto ret = iterator(*this);
  ret.pos = static_cast<char *>(this->pos) - this->stride * n;
  return ret;
}

template <typename T>
typename Slice<T>::iterator::difference_type
Slice<T>::iterator::operator-(const iterator &other) const noexcept {
  auto diff = std::distance(static_cast<char *>(other.pos),
                            static_cast<char *>(this->pos));
  return diff / this->stride;
}

template <typename T>
bool Slice<T>::iterator::operator==(const iterator &other) const noexcept {
  return this->pos == other.pos;
}

template <typename T>
bool Slice<T>::iterator::operator!=(const iterator &other) const noexcept {
  return this->pos != other.pos;
}

template <typename T>
bool Slice<T>::iterator::operator<(const iterator &other) const noexcept {
  return this->pos < other.pos;
}

template <typename T>
bool Slice<T>::iterator::operator<=(const iterator &other) const noexcept {
  return this->pos <= other.pos;
}

template <typename T>
bool Slice<T>::iterator::operator>(const iterator &other) const noexcept {
  return this->pos > other.pos;
}

template <typename T>
bool Slice<T>::iterator::operator>=(const iterator &other) const noexcept {
  return this->pos >= other.pos;
}

template <typename T>
typename Slice<T>::iterator Slice<T>::begin() const noexcept {
  iterator it;
  it.pos = slicePtr(this);
  it.stride = size_of<T>();
  return it;
}

template <typename T>
typename Slice<T>::iterator Slice<T>::end() const noexcept {
  iterator it = this->begin();
  it.pos = static_cast<char *>(it.pos) + it.stride * this->size();
  return it;
}

template <typename T>
void Slice<T>::swap(Slice &rhs) noexcept {
  std::swap(*this, rhs);
}
#endif // CXXBRIDGE1_RUST_SLICE

#ifndef CXXBRIDGE1_RUST_BOX
#define CXXBRIDGE1_RUST_BOX
template <typename T>
class Box final {
public:
  using element_type = T;
  using const_pointer =
      typename std::add_pointer<typename std::add_const<T>::type>::type;
  using pointer = typename std::add_pointer<T>::type;

  Box() = delete;
  Box(Box &&) noexcept;
  ~Box() noexcept;

  explicit Box(const T &);
  explicit Box(T &&);

  Box &operator=(Box &&) &noexcept;

  const T *operator->() const noexcept;
  const T &operator*() const noexcept;
  T *operator->() noexcept;
  T &operator*() noexcept;

  template <typename... Fields>
  static Box in_place(Fields &&...);

  void swap(Box &) noexcept;

  static Box from_raw(T *) noexcept;

  T *into_raw() noexcept;

  /* Deprecated */ using value_type = element_type;

private:
  class uninit;
  class allocation;
  Box(uninit) noexcept;
  void drop() noexcept;

  friend void swap(Box &lhs, Box &rhs) noexcept { lhs.swap(rhs); }

  T *ptr;
};

template <typename T>
class Box<T>::uninit {};

template <typename T>
class Box<T>::allocation {
  static T *alloc() noexcept;
  static void dealloc(T *) noexcept;

public:
  allocation() noexcept : ptr(alloc()) {}
  ~allocation() noexcept {
    if (this->ptr) {
      dealloc(this->ptr);
    }
  }
  T *ptr;
};

template <typename T>
Box<T>::Box(Box &&other) noexcept : ptr(other.ptr) {
  other.ptr = nullptr;
}

template <typename T>
Box<T>::Box(const T &val) {
  allocation alloc;
  ::new (alloc.ptr) T(val);
  this->ptr = alloc.ptr;
  alloc.ptr = nullptr;
}

template <typename T>
Box<T>::Box(T &&val) {
  allocation alloc;
  ::new (alloc.ptr) T(std::move(val));
  this->ptr = alloc.ptr;
  alloc.ptr = nullptr;
}

template <typename T>
Box<T>::~Box() noexcept {
  if (this->ptr) {
    this->drop();
  }
}

template <typename T>
Box<T> &Box<T>::operator=(Box &&other) &noexcept {
  if (this->ptr) {
    this->drop();
  }
  this->ptr = other.ptr;
  other.ptr = nullptr;
  return *this;
}

template <typename T>
const T *Box<T>::operator->() const noexcept {
  return this->ptr;
}

template <typename T>
const T &Box<T>::operator*() const noexcept {
  return *this->ptr;
}

template <typename T>
T *Box<T>::operator->() noexcept {
  return this->ptr;
}

template <typename T>
T &Box<T>::operator*() noexcept {
  return *this->ptr;
}

template <typename T>
template <typename... Fields>
Box<T> Box<T>::in_place(Fields &&...fields) {
  allocation alloc;
  auto ptr = alloc.ptr;
  ::new (ptr) T{std::forward<Fields>(fields)...};
  alloc.ptr = nullptr;
  return from_raw(ptr);
}

template <typename T>
void Box<T>::swap(Box &rhs) noexcept {
  using std::swap;
  swap(this->ptr, rhs.ptr);
}

template <typename T>
Box<T> Box<T>::from_raw(T *raw) noexcept {
  Box box = uninit{};
  box.ptr = raw;
  return box;
}

template <typename T>
T *Box<T>::into_raw() noexcept {
  T *raw = this->ptr;
  this->ptr = nullptr;
  return raw;
}

template <typename T>
Box<T>::Box(uninit) noexcept {}
#endif // CXXBRIDGE1_RUST_BOX

#ifndef CXXBRIDGE1_RUST_BITCOPY_T
#define CXXBRIDGE1_RUST_BITCOPY_T
struct unsafe_bitcopy_t final {
  explicit unsafe_bitcopy_t() = default;
};
#endif // CXXBRIDGE1_RUST_BITCOPY_T

#ifndef CXXBRIDGE1_RUST_VEC
#define CXXBRIDGE1_RUST_VEC
template <typename T>
class Vec final {
public:
  using value_type = T;

  Vec() noexcept;
  Vec(std::initializer_list<T>);
  Vec(const Vec &);
  Vec(Vec &&) noexcept;
  ~Vec() noexcept;

  Vec &operator=(Vec &&) &noexcept;
  Vec &operator=(const Vec &) &;

  std::size_t size() const noexcept;
  bool empty() const noexcept;
  const T *data() const noexcept;
  T *data() noexcept;
  std::size_t capacity() const noexcept;

  const T &operator[](std::size_t n) const noexcept;
  const T &at(std::size_t n) const;
  const T &front() const noexcept;
  const T &back() const noexcept;

  T &operator[](std::size_t n) noexcept;
  T &at(std::size_t n);
  T &front() noexcept;
  T &back() noexcept;

  void reserve(std::size_t new_cap);
  void push_back(const T &value);
  void push_back(T &&value);
  template <typename... Args>
  void emplace_back(Args &&...args);
  void truncate(std::size_t len);
  void clear();

  using iterator = typename Slice<T>::iterator;
  iterator begin() noexcept;
  iterator end() noexcept;

  using const_iterator = typename Slice<const T>::iterator;
  const_iterator begin() const noexcept;
  const_iterator end() const noexcept;
  const_iterator cbegin() const noexcept;
  const_iterator cend() const noexcept;

  void swap(Vec &) noexcept;

  Vec(unsafe_bitcopy_t, const Vec &) noexcept;

private:
  void reserve_total(std::size_t new_cap) noexcept;
  void set_len(std::size_t len) noexcept;
  void drop() noexcept;

  friend void swap(Vec &lhs, Vec &rhs) noexcept { lhs.swap(rhs); }

  std::array<std::uintptr_t, 3> repr;
};

template <typename T>
Vec<T>::Vec(std::initializer_list<T> init) : Vec{} {
  this->reserve_total(init.size());
  std::move(init.begin(), init.end(), std::back_inserter(*this));
}

template <typename T>
Vec<T>::Vec(const Vec &other) : Vec() {
  this->reserve_total(other.size());
  std::copy(other.begin(), other.end(), std::back_inserter(*this));
}

template <typename T>
Vec<T>::Vec(Vec &&other) noexcept : repr(other.repr) {
  new (&other) Vec();
}

template <typename T>
Vec<T>::~Vec() noexcept {
  this->drop();
}

template <typename T>
Vec<T> &Vec<T>::operator=(Vec &&other) &noexcept {
  this->drop();
  this->repr = other.repr;
  new (&other) Vec();
  return *this;
}

template <typename T>
Vec<T> &Vec<T>::operator=(const Vec &other) & {
  if (this != &other) {
    this->drop();
    new (this) Vec(other);
  }
  return *this;
}

template <typename T>
bool Vec<T>::empty() const noexcept {
  return this->size() == 0;
}

template <typename T>
T *Vec<T>::data() noexcept {
  return const_cast<T *>(const_cast<const Vec<T> *>(this)->data());
}

template <typename T>
const T &Vec<T>::operator[](std::size_t n) const noexcept {
  assert(n < this->size());
  auto data = reinterpret_cast<const char *>(this->data());
  return *reinterpret_cast<const T *>(data + n * size_of<T>());
}

template <typename T>
const T &Vec<T>::at(std::size_t n) const {
  if (n >= this->size()) {
    panic<std::out_of_range>("rust::Vec index out of range");
  }
  return (*this)[n];
}

template <typename T>
const T &Vec<T>::front() const noexcept {
  assert(!this->empty());
  return (*this)[0];
}

template <typename T>
const T &Vec<T>::back() const noexcept {
  assert(!this->empty());
  return (*this)[this->size() - 1];
}

template <typename T>
T &Vec<T>::operator[](std::size_t n) noexcept {
  assert(n < this->size());
  auto data = reinterpret_cast<char *>(this->data());
  return *reinterpret_cast<T *>(data + n * size_of<T>());
}

template <typename T>
T &Vec<T>::at(std::size_t n) {
  if (n >= this->size()) {
    panic<std::out_of_range>("rust::Vec index out of range");
  }
  return (*this)[n];
}

template <typename T>
T &Vec<T>::front() noexcept {
  assert(!this->empty());
  return (*this)[0];
}

template <typename T>
T &Vec<T>::back() noexcept {
  assert(!this->empty());
  return (*this)[this->size() - 1];
}

template <typename T>
void Vec<T>::reserve(std::size_t new_cap) {
  this->reserve_total(new_cap);
}

template <typename T>
void Vec<T>::push_back(const T &value) {
  this->emplace_back(value);
}

template <typename T>
void Vec<T>::push_back(T &&value) {
  this->emplace_back(std::move(value));
}

template <typename T>
template <typename... Args>
void Vec<T>::emplace_back(Args &&...args) {
  auto size = this->size();
  this->reserve_total(size + 1);
  ::new (reinterpret_cast<T *>(reinterpret_cast<char *>(this->data()) +
                               size * size_of<T>()))
      T(std::forward<Args>(args)...);
  this->set_len(size + 1);
}

template <typename T>
void Vec<T>::clear() {
  this->truncate(0);
}

template <typename T>
typename Vec<T>::iterator Vec<T>::begin() noexcept {
  return Slice<T>(this->data(), this->size()).begin();
}

template <typename T>
typename Vec<T>::iterator Vec<T>::end() noexcept {
  return Slice<T>(this->data(), this->size()).end();
}

template <typename T>
typename Vec<T>::const_iterator Vec<T>::begin() const noexcept {
  return this->cbegin();
}

template <typename T>
typename Vec<T>::const_iterator Vec<T>::end() const noexcept {
  return this->cend();
}

template <typename T>
typename Vec<T>::const_iterator Vec<T>::cbegin() const noexcept {
  return Slice<const T>(this->data(), this->size()).begin();
}

template <typename T>
typename Vec<T>::const_iterator Vec<T>::cend() const noexcept {
  return Slice<const T>(this->data(), this->size()).end();
}

template <typename T>
void Vec<T>::swap(Vec &rhs) noexcept {
  using std::swap;
  swap(this->repr, rhs.repr);
}

template <typename T>
Vec<T>::Vec(unsafe_bitcopy_t, const Vec &bits) noexcept : repr(bits.repr) {}
#endif // CXXBRIDGE1_RUST_VEC

#ifndef CXXBRIDGE1_RUST_ISIZE
#define CXXBRIDGE1_RUST_ISIZE
#if defined(_WIN32)
using isize = SSIZE_T;
#else
using isize = ssize_t;
#endif
#endif // CXXBRIDGE1_RUST_ISIZE

#ifndef CXXBRIDGE1_RUST_OPAQUE
#define CXXBRIDGE1_RUST_OPAQUE
class Opaque {
public:
  Opaque() = delete;
  Opaque(const Opaque &) = delete;
  ~Opaque() = delete;
};
#endif // CXXBRIDGE1_RUST_OPAQUE

#ifndef CXXBRIDGE1_IS_COMPLETE
#define CXXBRIDGE1_IS_COMPLETE
namespace detail {
namespace {
template <typename T, typename = std::size_t>
struct is_complete : std::false_type {};
template <typename T>
struct is_complete<T, decltype(sizeof(T))> : std::true_type {};
} // namespace
} // namespace detail
#endif // CXXBRIDGE1_IS_COMPLETE

#ifndef CXXBRIDGE1_LAYOUT
#define CXXBRIDGE1_LAYOUT
class layout {
  template <typename T>
  friend std::size_t size_of();
  template <typename T>
  friend std::size_t align_of();
  template <typename T>
  static typename std::enable_if<std::is_base_of<Opaque, T>::value,
                                 std::size_t>::type
  do_size_of() {
    return T::layout::size();
  }
  template <typename T>
  static typename std::enable_if<!std::is_base_of<Opaque, T>::value,
                                 std::size_t>::type
  do_size_of() {
    return sizeof(T);
  }
  template <typename T>
  static
      typename std::enable_if<detail::is_complete<T>::value, std::size_t>::type
      size_of() {
    return do_size_of<T>();
  }
  template <typename T>
  static typename std::enable_if<std::is_base_of<Opaque, T>::value,
                                 std::size_t>::type
  do_align_of() {
    return T::layout::align();
  }
  template <typename T>
  static typename std::enable_if<!std::is_base_of<Opaque, T>::value,
                                 std::size_t>::type
  do_align_of() {
    return alignof(T);
  }
  template <typename T>
  static
      typename std::enable_if<detail::is_complete<T>::value, std::size_t>::type
      align_of() {
    return do_align_of<T>();
  }
};

template <typename T>
std::size_t size_of() {
  return layout::size_of<T>();
}

template <typename T>
std::size_t align_of() {
  return layout::align_of<T>();
}
#endif // CXXBRIDGE1_LAYOUT

class Str::uninit {};
inline Str::Str(uninit) noexcept {}

template <typename T>
class Slice<T>::uninit {};
template <typename T>
inline Slice<T>::Slice(uninit) noexcept {}

namespace repr {
using Fat = ::std::array<::std::uintptr_t, 2>;
} // namespace repr

namespace detail {
template <typename T, typename = void *>
struct operator_new {
  void *operator()(::std::size_t sz) { return ::operator new(sz); }
};

template <typename T>
struct operator_new<T, decltype(T::operator new(sizeof(T)))> {
  void *operator()(::std::size_t sz) { return T::operator new(sz); }
};
} // namespace detail

template <typename T>
union MaybeUninit {
  T value;
  void *operator new(::std::size_t sz) { return detail::operator_new<T>{}(sz); }
  MaybeUninit() {}
  ~MaybeUninit() {}
};

namespace {
template <>
class impl<Str> final {
public:
  static Str new_unchecked(repr::Fat repr) noexcept {
    Str str = Str::uninit{};
    str.repr = repr;
    return str;
  }
};

template <typename T>
class impl<Slice<T>> final {
public:
  static Slice<T> slice(repr::Fat repr) noexcept {
    Slice<T> slice = typename Slice<T>::uninit{};
    slice.repr = repr;
    return slice;
  }
};
} // namespace
} // namespace cxxbridge1
} // namespace rust

namespace org {
  namespace nativescript {
    namespace canvas {
      struct FileHelper;
      struct Raf;
      enum class GLConstants : ::std::uint32_t;
      enum class InvalidateState : ::std::uint8_t;
      enum class PaintStyleType : ::std::uint8_t;
      enum class ImageBitmapPremultiplyAlpha : ::std::uint8_t;
      enum class ImageBitmapColorSpaceConversion : ::std::uint8_t;
      enum class ImageBitmapResizeQuality : ::std::uint8_t;
      struct CanvasRenderingContext2D;
      struct PaintStyle;
      struct TextMetrics;
      struct Path;
      struct Matrix;
      struct ImageData;
      struct ImageAsset;
      struct TextDecoder;
      struct TextEncoder;
      enum class WebGLExtensionType : ::std::uint8_t;
      enum class WebGLResultType : ::std::uint8_t;
      struct WebGLState;
      struct WebGLActiveInfo;
      struct WebGLResult;
      struct ContextAttributes;
      struct WebGLExtension;
      struct WebGLFramebufferAttachmentParameter;
      struct WebGLShaderPrecisionFormat;
      struct EXT_blend_minmax;
      struct EXT_color_buffer_half_float;
      struct EXT_disjoint_timer_query;
      struct EXT_sRGB;
      struct EXT_shader_texture_lod;
      struct EXT_texture_filter_anisotropic;
      struct OES_element_index_uint;
      struct OES_standard_derivatives;
      struct OES_texture_float;
      struct OES_texture_float_linear;
      struct OES_texture_half_float;
      struct OES_texture_half_float_linear;
      struct OES_vertex_array_object;
      struct WEBGL_color_buffer_float;
      struct WEBGL_compressed_texture_atc;
      struct WEBGL_compressed_texture_etc1;
      struct WEBGL_compressed_texture_s3tc;
      struct WEBGL_compressed_texture_s3tc_srgb;
      struct WEBGL_compressed_texture_etc;
      struct WEBGL_compressed_texture_pvrtc;
      struct WEBGL_lose_context;
      struct ANGLE_instanced_arrays;
      struct WEBGL_depth_texture;
      struct WEBGL_draw_buffers;
      struct WebGLSync;
      struct WebGLIndexedParameter;
    }
  }
}

namespace org {
namespace nativescript {
namespace canvas {
#ifndef CXXBRIDGE1_STRUCT_org$nativescript$canvas$FileHelper
#define CXXBRIDGE1_STRUCT_org$nativescript$canvas$FileHelper
struct FileHelper final : public ::rust::Opaque {
  ~FileHelper() = delete;

private:
  friend ::rust::layout;
  struct layout {
    static ::std::size_t size() noexcept;
    static ::std::size_t align() noexcept;
  };
};
#endif // CXXBRIDGE1_STRUCT_org$nativescript$canvas$FileHelper

#ifndef CXXBRIDGE1_STRUCT_org$nativescript$canvas$Raf
#define CXXBRIDGE1_STRUCT_org$nativescript$canvas$Raf
struct Raf final : public ::rust::Opaque {
  ~Raf() = delete;

private:
  friend ::rust::layout;
  struct layout {
    static ::std::size_t size() noexcept;
    static ::std::size_t align() noexcept;
  };
};
#endif // CXXBRIDGE1_STRUCT_org$nativescript$canvas$Raf

#ifndef CXXBRIDGE1_ENUM_org$nativescript$canvas$GLConstants
#define CXXBRIDGE1_ENUM_org$nativescript$canvas$GLConstants
enum class GLConstants : ::std::uint32_t {
  UNPACK_FLIP_Y_WEBGL = 37440,
  UNPACK_PREMULTIPLY_ALPHA_WEBGL = 37441,
  UNPACK_COLORSPACE_CONVERSION_WEBGL = 37443,
};
#endif // CXXBRIDGE1_ENUM_org$nativescript$canvas$GLConstants

#ifndef CXXBRIDGE1_ENUM_org$nativescript$canvas$InvalidateState
#define CXXBRIDGE1_ENUM_org$nativescript$canvas$InvalidateState
enum class InvalidateState : ::std::uint8_t {
  NONE = 0,
  PENDING = 1,
  INVALIDATING = 2,
};
#endif // CXXBRIDGE1_ENUM_org$nativescript$canvas$InvalidateState

#ifndef CXXBRIDGE1_ENUM_org$nativescript$canvas$PaintStyleType
#define CXXBRIDGE1_ENUM_org$nativescript$canvas$PaintStyleType
enum class PaintStyleType : ::std::uint8_t {
  None = 0,
  Color = 1,
  Gradient = 2,
  Pattern = 3,
};
#endif // CXXBRIDGE1_ENUM_org$nativescript$canvas$PaintStyleType

#ifndef CXXBRIDGE1_ENUM_org$nativescript$canvas$ImageBitmapPremultiplyAlpha
#define CXXBRIDGE1_ENUM_org$nativescript$canvas$ImageBitmapPremultiplyAlpha
enum class ImageBitmapPremultiplyAlpha : ::std::uint8_t {
  Default = 0,
  Premultiply = 1,
  None = 2,
};
#endif // CXXBRIDGE1_ENUM_org$nativescript$canvas$ImageBitmapPremultiplyAlpha

#ifndef CXXBRIDGE1_ENUM_org$nativescript$canvas$ImageBitmapColorSpaceConversion
#define CXXBRIDGE1_ENUM_org$nativescript$canvas$ImageBitmapColorSpaceConversion
enum class ImageBitmapColorSpaceConversion : ::std::uint8_t {
  Default = 0,
  None = 1,
};
#endif // CXXBRIDGE1_ENUM_org$nativescript$canvas$ImageBitmapColorSpaceConversion

#ifndef CXXBRIDGE1_ENUM_org$nativescript$canvas$ImageBitmapResizeQuality
#define CXXBRIDGE1_ENUM_org$nativescript$canvas$ImageBitmapResizeQuality
enum class ImageBitmapResizeQuality : ::std::uint8_t {
  Low = 0,
  Medium = 1,
  High = 2,
  Pixelated = 3,
};
#endif // CXXBRIDGE1_ENUM_org$nativescript$canvas$ImageBitmapResizeQuality

#ifndef CXXBRIDGE1_STRUCT_org$nativescript$canvas$CanvasRenderingContext2D
#define CXXBRIDGE1_STRUCT_org$nativescript$canvas$CanvasRenderingContext2D
struct CanvasRenderingContext2D final : public ::rust::Opaque {
  ~CanvasRenderingContext2D() = delete;

private:
  friend ::rust::layout;
  struct layout {
    static ::std::size_t size() noexcept;
    static ::std::size_t align() noexcept;
  };
};
#endif // CXXBRIDGE1_STRUCT_org$nativescript$canvas$CanvasRenderingContext2D

#ifndef CXXBRIDGE1_STRUCT_org$nativescript$canvas$PaintStyle
#define CXXBRIDGE1_STRUCT_org$nativescript$canvas$PaintStyle
struct PaintStyle final : public ::rust::Opaque {
  ~PaintStyle() = delete;

private:
  friend ::rust::layout;
  struct layout {
    static ::std::size_t size() noexcept;
    static ::std::size_t align() noexcept;
  };
};
#endif // CXXBRIDGE1_STRUCT_org$nativescript$canvas$PaintStyle

#ifndef CXXBRIDGE1_STRUCT_org$nativescript$canvas$TextMetrics
#define CXXBRIDGE1_STRUCT_org$nativescript$canvas$TextMetrics
struct TextMetrics final : public ::rust::Opaque {
  ~TextMetrics() = delete;

private:
  friend ::rust::layout;
  struct layout {
    static ::std::size_t size() noexcept;
    static ::std::size_t align() noexcept;
  };
};
#endif // CXXBRIDGE1_STRUCT_org$nativescript$canvas$TextMetrics

#ifndef CXXBRIDGE1_STRUCT_org$nativescript$canvas$Path
#define CXXBRIDGE1_STRUCT_org$nativescript$canvas$Path
struct Path final : public ::rust::Opaque {
  ~Path() = delete;

private:
  friend ::rust::layout;
  struct layout {
    static ::std::size_t size() noexcept;
    static ::std::size_t align() noexcept;
  };
};
#endif // CXXBRIDGE1_STRUCT_org$nativescript$canvas$Path

#ifndef CXXBRIDGE1_STRUCT_org$nativescript$canvas$Matrix
#define CXXBRIDGE1_STRUCT_org$nativescript$canvas$Matrix
struct Matrix final : public ::rust::Opaque {
  ~Matrix() = delete;

private:
  friend ::rust::layout;
  struct layout {
    static ::std::size_t size() noexcept;
    static ::std::size_t align() noexcept;
  };
};
#endif // CXXBRIDGE1_STRUCT_org$nativescript$canvas$Matrix

#ifndef CXXBRIDGE1_STRUCT_org$nativescript$canvas$ImageData
#define CXXBRIDGE1_STRUCT_org$nativescript$canvas$ImageData
struct ImageData final : public ::rust::Opaque {
  ~ImageData() = delete;

private:
  friend ::rust::layout;
  struct layout {
    static ::std::size_t size() noexcept;
    static ::std::size_t align() noexcept;
  };
};
#endif // CXXBRIDGE1_STRUCT_org$nativescript$canvas$ImageData

#ifndef CXXBRIDGE1_STRUCT_org$nativescript$canvas$ImageAsset
#define CXXBRIDGE1_STRUCT_org$nativescript$canvas$ImageAsset
struct ImageAsset final : public ::rust::Opaque {
  ~ImageAsset() = delete;

private:
  friend ::rust::layout;
  struct layout {
    static ::std::size_t size() noexcept;
    static ::std::size_t align() noexcept;
  };
};
#endif // CXXBRIDGE1_STRUCT_org$nativescript$canvas$ImageAsset

#ifndef CXXBRIDGE1_STRUCT_org$nativescript$canvas$TextDecoder
#define CXXBRIDGE1_STRUCT_org$nativescript$canvas$TextDecoder
struct TextDecoder final : public ::rust::Opaque {
  ~TextDecoder() = delete;

private:
  friend ::rust::layout;
  struct layout {
    static ::std::size_t size() noexcept;
    static ::std::size_t align() noexcept;
  };
};
#endif // CXXBRIDGE1_STRUCT_org$nativescript$canvas$TextDecoder

#ifndef CXXBRIDGE1_STRUCT_org$nativescript$canvas$TextEncoder
#define CXXBRIDGE1_STRUCT_org$nativescript$canvas$TextEncoder
struct TextEncoder final : public ::rust::Opaque {
  ~TextEncoder() = delete;

private:
  friend ::rust::layout;
  struct layout {
    static ::std::size_t size() noexcept;
    static ::std::size_t align() noexcept;
  };
};
#endif // CXXBRIDGE1_STRUCT_org$nativescript$canvas$TextEncoder

#ifndef CXXBRIDGE1_ENUM_org$nativescript$canvas$WebGLExtensionType
#define CXXBRIDGE1_ENUM_org$nativescript$canvas$WebGLExtensionType
enum class WebGLExtensionType : ::std::uint8_t {
  EXT_blend_minmax = 0,
  EXT_color_buffer_half_float = 1,
  EXT_disjoint_timer_query = 2,
  EXT_sRGB = 3,
  EXT_shader_texture_lod = 4,
  EXT_texture_filter_anisotropic = 5,
  OES_element_index_uint = 6,
  OES_standard_derivatives = 7,
  OES_texture_float = 8,
  OES_texture_float_linear = 9,
  OES_texture_half_float = 10,
  OES_texture_half_float_linear = 11,
  OES_vertex_array_object = 12,
  WEBGL_color_buffer_float = 13,
  WEBGL_compressed_texture_atc = 14,
  WEBGL_compressed_texture_etc1 = 15,
  WEBGL_compressed_texture_s3tc = 16,
  WEBGL_compressed_texture_s3tc_srgb = 17,
  WEBGL_compressed_texture_etc = 18,
  WEBGL_compressed_texture_pvrtc = 19,
  WEBGL_lose_context = 20,
  ANGLE_instanced_arrays = 21,
  WEBGL_depth_texture = 22,
  WEBGL_draw_buffers = 23,
  OES_fbo_render_mipmap = 24,
  None = 25,
};
#endif // CXXBRIDGE1_ENUM_org$nativescript$canvas$WebGLExtensionType

#ifndef CXXBRIDGE1_ENUM_org$nativescript$canvas$WebGLResultType
#define CXXBRIDGE1_ENUM_org$nativescript$canvas$WebGLResultType
enum class WebGLResultType : ::std::uint8_t {
  Boolean = 0,
  I32Array = 1,
  U32Array = 2,
  F32Array = 3,
  BooleanArray = 4,
  U32 = 5,
  I32 = 6,
  F32 = 7,
  String = 8,
  None = 9,
};
#endif // CXXBRIDGE1_ENUM_org$nativescript$canvas$WebGLResultType

#ifndef CXXBRIDGE1_STRUCT_org$nativescript$canvas$WebGLState
#define CXXBRIDGE1_STRUCT_org$nativescript$canvas$WebGLState
struct WebGLState final : public ::rust::Opaque {
  ~WebGLState() = delete;

private:
  friend ::rust::layout;
  struct layout {
    static ::std::size_t size() noexcept;
    static ::std::size_t align() noexcept;
  };
};
#endif // CXXBRIDGE1_STRUCT_org$nativescript$canvas$WebGLState

#ifndef CXXBRIDGE1_STRUCT_org$nativescript$canvas$WebGLActiveInfo
#define CXXBRIDGE1_STRUCT_org$nativescript$canvas$WebGLActiveInfo
struct WebGLActiveInfo final : public ::rust::Opaque {
  ~WebGLActiveInfo() = delete;

private:
  friend ::rust::layout;
  struct layout {
    static ::std::size_t size() noexcept;
    static ::std::size_t align() noexcept;
  };
};
#endif // CXXBRIDGE1_STRUCT_org$nativescript$canvas$WebGLActiveInfo

#ifndef CXXBRIDGE1_STRUCT_org$nativescript$canvas$WebGLResult
#define CXXBRIDGE1_STRUCT_org$nativescript$canvas$WebGLResult
struct WebGLResult final : public ::rust::Opaque {
  ~WebGLResult() = delete;

private:
  friend ::rust::layout;
  struct layout {
    static ::std::size_t size() noexcept;
    static ::std::size_t align() noexcept;
  };
};
#endif // CXXBRIDGE1_STRUCT_org$nativescript$canvas$WebGLResult

#ifndef CXXBRIDGE1_STRUCT_org$nativescript$canvas$ContextAttributes
#define CXXBRIDGE1_STRUCT_org$nativescript$canvas$ContextAttributes
struct ContextAttributes final : public ::rust::Opaque {
  ~ContextAttributes() = delete;

private:
  friend ::rust::layout;
  struct layout {
    static ::std::size_t size() noexcept;
    static ::std::size_t align() noexcept;
  };
};
#endif // CXXBRIDGE1_STRUCT_org$nativescript$canvas$ContextAttributes

#ifndef CXXBRIDGE1_STRUCT_org$nativescript$canvas$WebGLExtension
#define CXXBRIDGE1_STRUCT_org$nativescript$canvas$WebGLExtension
struct WebGLExtension final : public ::rust::Opaque {
  ~WebGLExtension() = delete;

private:
  friend ::rust::layout;
  struct layout {
    static ::std::size_t size() noexcept;
    static ::std::size_t align() noexcept;
  };
};
#endif // CXXBRIDGE1_STRUCT_org$nativescript$canvas$WebGLExtension

#ifndef CXXBRIDGE1_STRUCT_org$nativescript$canvas$WebGLFramebufferAttachmentParameter
#define CXXBRIDGE1_STRUCT_org$nativescript$canvas$WebGLFramebufferAttachmentParameter
struct WebGLFramebufferAttachmentParameter final : public ::rust::Opaque {
  ~WebGLFramebufferAttachmentParameter() = delete;

private:
  friend ::rust::layout;
  struct layout {
    static ::std::size_t size() noexcept;
    static ::std::size_t align() noexcept;
  };
};
#endif // CXXBRIDGE1_STRUCT_org$nativescript$canvas$WebGLFramebufferAttachmentParameter

#ifndef CXXBRIDGE1_STRUCT_org$nativescript$canvas$WebGLShaderPrecisionFormat
#define CXXBRIDGE1_STRUCT_org$nativescript$canvas$WebGLShaderPrecisionFormat
struct WebGLShaderPrecisionFormat final : public ::rust::Opaque {
  ~WebGLShaderPrecisionFormat() = delete;

private:
  friend ::rust::layout;
  struct layout {
    static ::std::size_t size() noexcept;
    static ::std::size_t align() noexcept;
  };
};
#endif // CXXBRIDGE1_STRUCT_org$nativescript$canvas$WebGLShaderPrecisionFormat

#ifndef CXXBRIDGE1_STRUCT_org$nativescript$canvas$EXT_blend_minmax
#define CXXBRIDGE1_STRUCT_org$nativescript$canvas$EXT_blend_minmax
struct EXT_blend_minmax final : public ::rust::Opaque {
  ~EXT_blend_minmax() = delete;

private:
  friend ::rust::layout;
  struct layout {
    static ::std::size_t size() noexcept;
    static ::std::size_t align() noexcept;
  };
};
#endif // CXXBRIDGE1_STRUCT_org$nativescript$canvas$EXT_blend_minmax

#ifndef CXXBRIDGE1_STRUCT_org$nativescript$canvas$EXT_color_buffer_half_float
#define CXXBRIDGE1_STRUCT_org$nativescript$canvas$EXT_color_buffer_half_float
struct EXT_color_buffer_half_float final : public ::rust::Opaque {
  ~EXT_color_buffer_half_float() = delete;

private:
  friend ::rust::layout;
  struct layout {
    static ::std::size_t size() noexcept;
    static ::std::size_t align() noexcept;
  };
};
#endif // CXXBRIDGE1_STRUCT_org$nativescript$canvas$EXT_color_buffer_half_float

#ifndef CXXBRIDGE1_STRUCT_org$nativescript$canvas$EXT_disjoint_timer_query
#define CXXBRIDGE1_STRUCT_org$nativescript$canvas$EXT_disjoint_timer_query
struct EXT_disjoint_timer_query final : public ::rust::Opaque {
  ~EXT_disjoint_timer_query() = delete;

private:
  friend ::rust::layout;
  struct layout {
    static ::std::size_t size() noexcept;
    static ::std::size_t align() noexcept;
  };
};
#endif // CXXBRIDGE1_STRUCT_org$nativescript$canvas$EXT_disjoint_timer_query

#ifndef CXXBRIDGE1_STRUCT_org$nativescript$canvas$EXT_sRGB
#define CXXBRIDGE1_STRUCT_org$nativescript$canvas$EXT_sRGB
struct EXT_sRGB final : public ::rust::Opaque {
  ~EXT_sRGB() = delete;

private:
  friend ::rust::layout;
  struct layout {
    static ::std::size_t size() noexcept;
    static ::std::size_t align() noexcept;
  };
};
#endif // CXXBRIDGE1_STRUCT_org$nativescript$canvas$EXT_sRGB

#ifndef CXXBRIDGE1_STRUCT_org$nativescript$canvas$EXT_shader_texture_lod
#define CXXBRIDGE1_STRUCT_org$nativescript$canvas$EXT_shader_texture_lod
struct EXT_shader_texture_lod final : public ::rust::Opaque {
  ~EXT_shader_texture_lod() = delete;

private:
  friend ::rust::layout;
  struct layout {
    static ::std::size_t size() noexcept;
    static ::std::size_t align() noexcept;
  };
};
#endif // CXXBRIDGE1_STRUCT_org$nativescript$canvas$EXT_shader_texture_lod

#ifndef CXXBRIDGE1_STRUCT_org$nativescript$canvas$EXT_texture_filter_anisotropic
#define CXXBRIDGE1_STRUCT_org$nativescript$canvas$EXT_texture_filter_anisotropic
struct EXT_texture_filter_anisotropic final : public ::rust::Opaque {
  ~EXT_texture_filter_anisotropic() = delete;

private:
  friend ::rust::layout;
  struct layout {
    static ::std::size_t size() noexcept;
    static ::std::size_t align() noexcept;
  };
};
#endif // CXXBRIDGE1_STRUCT_org$nativescript$canvas$EXT_texture_filter_anisotropic

#ifndef CXXBRIDGE1_STRUCT_org$nativescript$canvas$OES_element_index_uint
#define CXXBRIDGE1_STRUCT_org$nativescript$canvas$OES_element_index_uint
struct OES_element_index_uint final : public ::rust::Opaque {
  ~OES_element_index_uint() = delete;

private:
  friend ::rust::layout;
  struct layout {
    static ::std::size_t size() noexcept;
    static ::std::size_t align() noexcept;
  };
};
#endif // CXXBRIDGE1_STRUCT_org$nativescript$canvas$OES_element_index_uint

#ifndef CXXBRIDGE1_STRUCT_org$nativescript$canvas$OES_standard_derivatives
#define CXXBRIDGE1_STRUCT_org$nativescript$canvas$OES_standard_derivatives
struct OES_standard_derivatives final : public ::rust::Opaque {
  ~OES_standard_derivatives() = delete;

private:
  friend ::rust::layout;
  struct layout {
    static ::std::size_t size() noexcept;
    static ::std::size_t align() noexcept;
  };
};
#endif // CXXBRIDGE1_STRUCT_org$nativescript$canvas$OES_standard_derivatives

#ifndef CXXBRIDGE1_STRUCT_org$nativescript$canvas$OES_texture_float
#define CXXBRIDGE1_STRUCT_org$nativescript$canvas$OES_texture_float
struct OES_texture_float final : public ::rust::Opaque {
  ~OES_texture_float() = delete;

private:
  friend ::rust::layout;
  struct layout {
    static ::std::size_t size() noexcept;
    static ::std::size_t align() noexcept;
  };
};
#endif // CXXBRIDGE1_STRUCT_org$nativescript$canvas$OES_texture_float

#ifndef CXXBRIDGE1_STRUCT_org$nativescript$canvas$OES_texture_float_linear
#define CXXBRIDGE1_STRUCT_org$nativescript$canvas$OES_texture_float_linear
struct OES_texture_float_linear final : public ::rust::Opaque {
  ~OES_texture_float_linear() = delete;

private:
  friend ::rust::layout;
  struct layout {
    static ::std::size_t size() noexcept;
    static ::std::size_t align() noexcept;
  };
};
#endif // CXXBRIDGE1_STRUCT_org$nativescript$canvas$OES_texture_float_linear

#ifndef CXXBRIDGE1_STRUCT_org$nativescript$canvas$OES_texture_half_float
#define CXXBRIDGE1_STRUCT_org$nativescript$canvas$OES_texture_half_float
struct OES_texture_half_float final : public ::rust::Opaque {
  ~OES_texture_half_float() = delete;

private:
  friend ::rust::layout;
  struct layout {
    static ::std::size_t size() noexcept;
    static ::std::size_t align() noexcept;
  };
};
#endif // CXXBRIDGE1_STRUCT_org$nativescript$canvas$OES_texture_half_float

#ifndef CXXBRIDGE1_STRUCT_org$nativescript$canvas$OES_texture_half_float_linear
#define CXXBRIDGE1_STRUCT_org$nativescript$canvas$OES_texture_half_float_linear
struct OES_texture_half_float_linear final : public ::rust::Opaque {
  ~OES_texture_half_float_linear() = delete;

private:
  friend ::rust::layout;
  struct layout {
    static ::std::size_t size() noexcept;
    static ::std::size_t align() noexcept;
  };
};
#endif // CXXBRIDGE1_STRUCT_org$nativescript$canvas$OES_texture_half_float_linear

#ifndef CXXBRIDGE1_STRUCT_org$nativescript$canvas$OES_vertex_array_object
#define CXXBRIDGE1_STRUCT_org$nativescript$canvas$OES_vertex_array_object
struct OES_vertex_array_object final : public ::rust::Opaque {
  ~OES_vertex_array_object() = delete;

private:
  friend ::rust::layout;
  struct layout {
    static ::std::size_t size() noexcept;
    static ::std::size_t align() noexcept;
  };
};
#endif // CXXBRIDGE1_STRUCT_org$nativescript$canvas$OES_vertex_array_object

#ifndef CXXBRIDGE1_STRUCT_org$nativescript$canvas$WEBGL_color_buffer_float
#define CXXBRIDGE1_STRUCT_org$nativescript$canvas$WEBGL_color_buffer_float
struct WEBGL_color_buffer_float final : public ::rust::Opaque {
  ~WEBGL_color_buffer_float() = delete;

private:
  friend ::rust::layout;
  struct layout {
    static ::std::size_t size() noexcept;
    static ::std::size_t align() noexcept;
  };
};
#endif // CXXBRIDGE1_STRUCT_org$nativescript$canvas$WEBGL_color_buffer_float

#ifndef CXXBRIDGE1_STRUCT_org$nativescript$canvas$WEBGL_compressed_texture_atc
#define CXXBRIDGE1_STRUCT_org$nativescript$canvas$WEBGL_compressed_texture_atc
struct WEBGL_compressed_texture_atc final : public ::rust::Opaque {
  ~WEBGL_compressed_texture_atc() = delete;

private:
  friend ::rust::layout;
  struct layout {
    static ::std::size_t size() noexcept;
    static ::std::size_t align() noexcept;
  };
};
#endif // CXXBRIDGE1_STRUCT_org$nativescript$canvas$WEBGL_compressed_texture_atc

#ifndef CXXBRIDGE1_STRUCT_org$nativescript$canvas$WEBGL_compressed_texture_etc1
#define CXXBRIDGE1_STRUCT_org$nativescript$canvas$WEBGL_compressed_texture_etc1
struct WEBGL_compressed_texture_etc1 final : public ::rust::Opaque {
  ~WEBGL_compressed_texture_etc1() = delete;

private:
  friend ::rust::layout;
  struct layout {
    static ::std::size_t size() noexcept;
    static ::std::size_t align() noexcept;
  };
};
#endif // CXXBRIDGE1_STRUCT_org$nativescript$canvas$WEBGL_compressed_texture_etc1

#ifndef CXXBRIDGE1_STRUCT_org$nativescript$canvas$WEBGL_compressed_texture_s3tc
#define CXXBRIDGE1_STRUCT_org$nativescript$canvas$WEBGL_compressed_texture_s3tc
struct WEBGL_compressed_texture_s3tc final : public ::rust::Opaque {
  ~WEBGL_compressed_texture_s3tc() = delete;

private:
  friend ::rust::layout;
  struct layout {
    static ::std::size_t size() noexcept;
    static ::std::size_t align() noexcept;
  };
};
#endif // CXXBRIDGE1_STRUCT_org$nativescript$canvas$WEBGL_compressed_texture_s3tc

#ifndef CXXBRIDGE1_STRUCT_org$nativescript$canvas$WEBGL_compressed_texture_s3tc_srgb
#define CXXBRIDGE1_STRUCT_org$nativescript$canvas$WEBGL_compressed_texture_s3tc_srgb
struct WEBGL_compressed_texture_s3tc_srgb final : public ::rust::Opaque {
  ~WEBGL_compressed_texture_s3tc_srgb() = delete;

private:
  friend ::rust::layout;
  struct layout {
    static ::std::size_t size() noexcept;
    static ::std::size_t align() noexcept;
  };
};
#endif // CXXBRIDGE1_STRUCT_org$nativescript$canvas$WEBGL_compressed_texture_s3tc_srgb

#ifndef CXXBRIDGE1_STRUCT_org$nativescript$canvas$WEBGL_compressed_texture_etc
#define CXXBRIDGE1_STRUCT_org$nativescript$canvas$WEBGL_compressed_texture_etc
struct WEBGL_compressed_texture_etc final : public ::rust::Opaque {
  ~WEBGL_compressed_texture_etc() = delete;

private:
  friend ::rust::layout;
  struct layout {
    static ::std::size_t size() noexcept;
    static ::std::size_t align() noexcept;
  };
};
#endif // CXXBRIDGE1_STRUCT_org$nativescript$canvas$WEBGL_compressed_texture_etc

#ifndef CXXBRIDGE1_STRUCT_org$nativescript$canvas$WEBGL_compressed_texture_pvrtc
#define CXXBRIDGE1_STRUCT_org$nativescript$canvas$WEBGL_compressed_texture_pvrtc
struct WEBGL_compressed_texture_pvrtc final : public ::rust::Opaque {
  ~WEBGL_compressed_texture_pvrtc() = delete;

private:
  friend ::rust::layout;
  struct layout {
    static ::std::size_t size() noexcept;
    static ::std::size_t align() noexcept;
  };
};
#endif // CXXBRIDGE1_STRUCT_org$nativescript$canvas$WEBGL_compressed_texture_pvrtc

#ifndef CXXBRIDGE1_STRUCT_org$nativescript$canvas$WEBGL_lose_context
#define CXXBRIDGE1_STRUCT_org$nativescript$canvas$WEBGL_lose_context
struct WEBGL_lose_context final : public ::rust::Opaque {
  ~WEBGL_lose_context() = delete;

private:
  friend ::rust::layout;
  struct layout {
    static ::std::size_t size() noexcept;
    static ::std::size_t align() noexcept;
  };
};
#endif // CXXBRIDGE1_STRUCT_org$nativescript$canvas$WEBGL_lose_context

#ifndef CXXBRIDGE1_STRUCT_org$nativescript$canvas$ANGLE_instanced_arrays
#define CXXBRIDGE1_STRUCT_org$nativescript$canvas$ANGLE_instanced_arrays
struct ANGLE_instanced_arrays final : public ::rust::Opaque {
  ~ANGLE_instanced_arrays() = delete;

private:
  friend ::rust::layout;
  struct layout {
    static ::std::size_t size() noexcept;
    static ::std::size_t align() noexcept;
  };
};
#endif // CXXBRIDGE1_STRUCT_org$nativescript$canvas$ANGLE_instanced_arrays

#ifndef CXXBRIDGE1_STRUCT_org$nativescript$canvas$WEBGL_depth_texture
#define CXXBRIDGE1_STRUCT_org$nativescript$canvas$WEBGL_depth_texture
struct WEBGL_depth_texture final : public ::rust::Opaque {
  ~WEBGL_depth_texture() = delete;

private:
  friend ::rust::layout;
  struct layout {
    static ::std::size_t size() noexcept;
    static ::std::size_t align() noexcept;
  };
};
#endif // CXXBRIDGE1_STRUCT_org$nativescript$canvas$WEBGL_depth_texture

#ifndef CXXBRIDGE1_STRUCT_org$nativescript$canvas$WEBGL_draw_buffers
#define CXXBRIDGE1_STRUCT_org$nativescript$canvas$WEBGL_draw_buffers
struct WEBGL_draw_buffers final : public ::rust::Opaque {
  ~WEBGL_draw_buffers() = delete;

private:
  friend ::rust::layout;
  struct layout {
    static ::std::size_t size() noexcept;
    static ::std::size_t align() noexcept;
  };
};
#endif // CXXBRIDGE1_STRUCT_org$nativescript$canvas$WEBGL_draw_buffers

#ifndef CXXBRIDGE1_STRUCT_org$nativescript$canvas$WebGLSync
#define CXXBRIDGE1_STRUCT_org$nativescript$canvas$WebGLSync
struct WebGLSync final : public ::rust::Opaque {
  ~WebGLSync() = delete;

private:
  friend ::rust::layout;
  struct layout {
    static ::std::size_t size() noexcept;
    static ::std::size_t align() noexcept;
  };
};
#endif // CXXBRIDGE1_STRUCT_org$nativescript$canvas$WebGLSync

#ifndef CXXBRIDGE1_STRUCT_org$nativescript$canvas$WebGLIndexedParameter
#define CXXBRIDGE1_STRUCT_org$nativescript$canvas$WebGLIndexedParameter
struct WebGLIndexedParameter final : public ::rust::Opaque {
  ~WebGLIndexedParameter() = delete;

private:
  friend ::rust::layout;
  struct layout {
    static ::std::size_t size() noexcept;
    static ::std::size_t align() noexcept;
  };
};
#endif // CXXBRIDGE1_STRUCT_org$nativescript$canvas$WebGLIndexedParameter

extern "C" {
::std::size_t org$nativescript$canvas$cxxbridge1$FileHelper$operator$sizeof() noexcept;
::std::size_t org$nativescript$canvas$cxxbridge1$FileHelper$operator$alignof() noexcept;

::org::nativescript::canvas::FileHelper *org$nativescript$canvas$cxxbridge1$canvas_native_helper_read_file(::rust::Str path) noexcept;

bool org$nativescript$canvas$cxxbridge1$canvas_native_helper_read_file_has_error(::org::nativescript::canvas::FileHelper const &file) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_helper_read_file_get_data(::org::nativescript::canvas::FileHelper *file, ::rust::Vec<::std::uint8_t> *return$) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_helper_read_file_get_error(::org::nativescript::canvas::FileHelper const &file, ::rust::String *return$) noexcept;
::std::size_t org$nativescript$canvas$cxxbridge1$Raf$operator$sizeof() noexcept;
::std::size_t org$nativescript$canvas$cxxbridge1$Raf$operator$alignof() noexcept;

::org::nativescript::canvas::Raf *org$nativescript$canvas$cxxbridge1$canvas_native_raf_create(::rust::isize callback) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_raf_start(::org::nativescript::canvas::Raf &raf) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_raf_stop(::org::nativescript::canvas::Raf &raf) noexcept;

bool org$nativescript$canvas$cxxbridge1$canvas_native_raf_get_started(::org::nativescript::canvas::Raf const &raf) noexcept;
} // extern "C"
} // namespace canvas
} // namespace nativescript
} // namespace org

extern "C" {
void cxxbridge1$OnRafCallbackOnFrame(::rust::isize callback, ::std::int64_t ts) noexcept {
  void (*OnRafCallbackOnFrame$)(::rust::isize, ::std::int64_t) = ::OnRafCallbackOnFrame;
  OnRafCallbackOnFrame$(callback, ts);
}
} // extern "C"

namespace org {
namespace nativescript {
namespace canvas {
extern "C" {
::std::size_t org$nativescript$canvas$cxxbridge1$CanvasRenderingContext2D$operator$sizeof() noexcept;
::std::size_t org$nativescript$canvas$cxxbridge1$CanvasRenderingContext2D$operator$alignof() noexcept;
::std::size_t org$nativescript$canvas$cxxbridge1$PaintStyle$operator$sizeof() noexcept;
::std::size_t org$nativescript$canvas$cxxbridge1$PaintStyle$operator$alignof() noexcept;
::std::size_t org$nativescript$canvas$cxxbridge1$TextMetrics$operator$sizeof() noexcept;
::std::size_t org$nativescript$canvas$cxxbridge1$TextMetrics$operator$alignof() noexcept;
::std::size_t org$nativescript$canvas$cxxbridge1$Path$operator$sizeof() noexcept;
::std::size_t org$nativescript$canvas$cxxbridge1$Path$operator$alignof() noexcept;
::std::size_t org$nativescript$canvas$cxxbridge1$Matrix$operator$sizeof() noexcept;
::std::size_t org$nativescript$canvas$cxxbridge1$Matrix$operator$alignof() noexcept;
::std::size_t org$nativescript$canvas$cxxbridge1$ImageData$operator$sizeof() noexcept;
::std::size_t org$nativescript$canvas$cxxbridge1$ImageData$operator$alignof() noexcept;
::std::size_t org$nativescript$canvas$cxxbridge1$ImageAsset$operator$sizeof() noexcept;
::std::size_t org$nativescript$canvas$cxxbridge1$ImageAsset$operator$alignof() noexcept;
::std::size_t org$nativescript$canvas$cxxbridge1$TextDecoder$operator$sizeof() noexcept;
::std::size_t org$nativescript$canvas$cxxbridge1$TextDecoder$operator$alignof() noexcept;
::std::size_t org$nativescript$canvas$cxxbridge1$TextEncoder$operator$sizeof() noexcept;
::std::size_t org$nativescript$canvas$cxxbridge1$TextEncoder$operator$alignof() noexcept;

void org$nativescript$canvas$cxxbridge1$console_log(::rust::Str text) noexcept;

void org$nativescript$canvas$cxxbridge1$str_to_buf(::rust::Str value, ::rust::Vec<::std::uint8_t> *return$) noexcept;

::org::nativescript::canvas::ImageAsset *org$nativescript$canvas$cxxbridge1$canvas_native_image_bitmap_create_from_asset(::org::nativescript::canvas::ImageAsset &asset, bool flip_y, ::org::nativescript::canvas::ImageBitmapPremultiplyAlpha premultiply_alpha, ::org::nativescript::canvas::ImageBitmapColorSpaceConversion color_space_conversion, ::org::nativescript::canvas::ImageBitmapResizeQuality resize_quality, float resize_width, float resize_height) noexcept;

::org::nativescript::canvas::ImageAsset *org$nativescript$canvas$cxxbridge1$canvas_native_image_bitmap_create_from_asset_src_rect(::org::nativescript::canvas::ImageAsset &asset, float sx, float sy, float s_width, float s_height, bool flip_y, ::org::nativescript::canvas::ImageBitmapPremultiplyAlpha premultiply_alpha, ::org::nativescript::canvas::ImageBitmapColorSpaceConversion color_space_conversion, ::org::nativescript::canvas::ImageBitmapResizeQuality resize_quality, float resize_width, float resize_height) noexcept;

::org::nativescript::canvas::ImageAsset *org$nativescript$canvas$cxxbridge1$canvas_native_image_bitmap_create_from_encoded_bytes(::rust::Slice<::std::uint8_t const> bytes, bool flip_y, ::org::nativescript::canvas::ImageBitmapPremultiplyAlpha premultiply_alpha, ::org::nativescript::canvas::ImageBitmapColorSpaceConversion color_space_conversion, ::org::nativescript::canvas::ImageBitmapResizeQuality resize_quality, float resize_width, float resize_height) noexcept;

bool org$nativescript$canvas$cxxbridge1$canvas_native_image_bitmap_create_from_encoded_bytes_with_output(::rust::Slice<::std::uint8_t const> bytes, bool flip_y, ::org::nativescript::canvas::ImageBitmapPremultiplyAlpha premultiply_alpha, ::org::nativescript::canvas::ImageBitmapColorSpaceConversion color_space_conversion, ::org::nativescript::canvas::ImageBitmapResizeQuality resize_quality, float resize_width, float resize_height, ::org::nativescript::canvas::ImageAsset &output) noexcept;

::org::nativescript::canvas::ImageAsset *org$nativescript$canvas$cxxbridge1$canvas_native_image_bitmap_create_from_encoded_bytes_src_rect(::rust::Slice<::std::uint8_t const> bytes, float sx, float sy, float s_width, float s_height, bool flip_y, ::org::nativescript::canvas::ImageBitmapPremultiplyAlpha premultiply_alpha, ::org::nativescript::canvas::ImageBitmapColorSpaceConversion color_space_conversion, ::org::nativescript::canvas::ImageBitmapResizeQuality resize_quality, float resize_width, float resize_height) noexcept;

bool org$nativescript$canvas$cxxbridge1$canvas_native_image_bitmap_create_from_encoded_bytes_src_rect_with_output(::rust::Slice<::std::uint8_t const> bytes, float sx, float sy, float s_width, float s_height, bool flip_y, ::org::nativescript::canvas::ImageBitmapPremultiplyAlpha premultiply_alpha, ::org::nativescript::canvas::ImageBitmapColorSpaceConversion color_space_conversion, ::org::nativescript::canvas::ImageBitmapResizeQuality resize_quality, float resize_width, float resize_height, ::org::nativescript::canvas::ImageAsset &output) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_path_add_path(::org::nativescript::canvas::Path &path, ::org::nativescript::canvas::Path const &path_to_add) noexcept;

::org::nativescript::canvas::Path *org$nativescript$canvas$cxxbridge1$canvas_native_path_create() noexcept;

::org::nativescript::canvas::Path *org$nativescript$canvas$cxxbridge1$canvas_native_path_create_with_path(::org::nativescript::canvas::Path const &path) noexcept;

::org::nativescript::canvas::Path *org$nativescript$canvas$cxxbridge1$canvas_native_path_create_with_string(::rust::String *string) noexcept;

::org::nativescript::canvas::Path *org$nativescript$canvas$cxxbridge1$canvas_native_path_create_with_str(::rust::Str string) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_path_close_path(::org::nativescript::canvas::Path &path) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_path_move_to(::org::nativescript::canvas::Path &path, float x, float y) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_path_line_to(::org::nativescript::canvas::Path &path, float x, float y) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_path_bezier_curve_to(::org::nativescript::canvas::Path &path, float cp1x, float cp1y, float cp2x, float cp2y, float x, float y) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_path_quadratic_curve_to(::org::nativescript::canvas::Path &path, float cpx, float cpy, float x, float y) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_path_arc(::org::nativescript::canvas::Path &path, float x, float y, float radius, float start_angle, float end_angle, bool anti_clockwise) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_path_arc_to(::org::nativescript::canvas::Path &path, float x1, float y1, float x2, float y2, float radius) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_path_ellipse(::org::nativescript::canvas::Path &path, float x, float y, float radius_x, float radius_y, float rotation, float start_angle, float end_angle, bool anticlockwise) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_path_rect(::org::nativescript::canvas::Path &path, float x, float y, float width, float height) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_path_round_rect(::org::nativescript::canvas::Path &path, float x, float y, float width, float height, ::rust::Slice<float const> radii) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_path_round_rect_tl_tr_br_bl(::org::nativescript::canvas::Path &path, float x, float y, float width, float height, float top_left, float top_right, float bottom_right, float bottom_left) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_path_to_string(::org::nativescript::canvas::Path const &path, ::rust::String *return$) noexcept;

::org::nativescript::canvas::Matrix *org$nativescript$canvas$cxxbridge1$canvas_native_matrix_create() noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_matrix_update(::org::nativescript::canvas::Matrix &matrix, ::rust::Slice<float const> slice) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_matrix_update_3d(::org::nativescript::canvas::Matrix &matrix, ::std::array<float, 16> const &slice) noexcept;

float org$nativescript$canvas$cxxbridge1$canvas_native_matrix_get_a(::org::nativescript::canvas::Matrix const &matrix) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_matrix_set_a(::org::nativescript::canvas::Matrix &matrix, float a) noexcept;

float org$nativescript$canvas$cxxbridge1$canvas_native_matrix_get_b(::org::nativescript::canvas::Matrix const &matrix) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_matrix_set_b(::org::nativescript::canvas::Matrix &matrix, float b) noexcept;

float org$nativescript$canvas$cxxbridge1$canvas_native_matrix_get_c(::org::nativescript::canvas::Matrix const &matrix) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_matrix_set_c(::org::nativescript::canvas::Matrix &matrix, float c) noexcept;

float org$nativescript$canvas$cxxbridge1$canvas_native_matrix_get_d(::org::nativescript::canvas::Matrix const &matrix) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_matrix_set_d(::org::nativescript::canvas::Matrix &matrix, float d) noexcept;

float org$nativescript$canvas$cxxbridge1$canvas_native_matrix_get_e(::org::nativescript::canvas::Matrix const &matrix) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_matrix_set_e(::org::nativescript::canvas::Matrix &matrix, float e) noexcept;

float org$nativescript$canvas$cxxbridge1$canvas_native_matrix_get_f(::org::nativescript::canvas::Matrix const &matrix) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_matrix_set_f(::org::nativescript::canvas::Matrix &matrix, float f) noexcept;

float org$nativescript$canvas$cxxbridge1$canvas_native_matrix_get_m11(::org::nativescript::canvas::Matrix const &matrix) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_matrix_set_m11(::org::nativescript::canvas::Matrix &matrix, float m11) noexcept;

float org$nativescript$canvas$cxxbridge1$canvas_native_matrix_get_m12(::org::nativescript::canvas::Matrix const &matrix) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_matrix_set_m12(::org::nativescript::canvas::Matrix &matrix, float m12) noexcept;

float org$nativescript$canvas$cxxbridge1$canvas_native_matrix_get_m13(::org::nativescript::canvas::Matrix const &matrix) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_matrix_set_m13(::org::nativescript::canvas::Matrix &matrix, float m13) noexcept;

float org$nativescript$canvas$cxxbridge1$canvas_native_matrix_get_m14(::org::nativescript::canvas::Matrix const &matrix) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_matrix_set_m14(::org::nativescript::canvas::Matrix &matrix, float m14) noexcept;

float org$nativescript$canvas$cxxbridge1$canvas_native_matrix_get_m21(::org::nativescript::canvas::Matrix const &matrix) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_matrix_set_m21(::org::nativescript::canvas::Matrix &matrix, float m21) noexcept;

float org$nativescript$canvas$cxxbridge1$canvas_native_matrix_get_m22(::org::nativescript::canvas::Matrix const &matrix) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_matrix_set_m22(::org::nativescript::canvas::Matrix &matrix, float m22) noexcept;

float org$nativescript$canvas$cxxbridge1$canvas_native_matrix_get_m23(::org::nativescript::canvas::Matrix const &matrix) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_matrix_set_m23(::org::nativescript::canvas::Matrix &matrix, float m23) noexcept;

float org$nativescript$canvas$cxxbridge1$canvas_native_matrix_get_m24(::org::nativescript::canvas::Matrix const &matrix) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_matrix_set_m24(::org::nativescript::canvas::Matrix &matrix, float m24) noexcept;

float org$nativescript$canvas$cxxbridge1$canvas_native_matrix_get_m31(::org::nativescript::canvas::Matrix const &matrix) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_matrix_set_m31(::org::nativescript::canvas::Matrix &matrix, float m31) noexcept;

float org$nativescript$canvas$cxxbridge1$canvas_native_matrix_get_m32(::org::nativescript::canvas::Matrix const &matrix) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_matrix_set_m32(::org::nativescript::canvas::Matrix &matrix, float m32) noexcept;

float org$nativescript$canvas$cxxbridge1$canvas_native_matrix_get_m33(::org::nativescript::canvas::Matrix const &matrix) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_matrix_set_m33(::org::nativescript::canvas::Matrix &matrix, float m33) noexcept;

float org$nativescript$canvas$cxxbridge1$canvas_native_matrix_get_m34(::org::nativescript::canvas::Matrix const &matrix) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_matrix_set_m34(::org::nativescript::canvas::Matrix &matrix, float m34) noexcept;

float org$nativescript$canvas$cxxbridge1$canvas_native_matrix_get_m41(::org::nativescript::canvas::Matrix const &matrix) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_matrix_set_m41(::org::nativescript::canvas::Matrix &matrix, float m41) noexcept;

float org$nativescript$canvas$cxxbridge1$canvas_native_matrix_get_m42(::org::nativescript::canvas::Matrix const &matrix) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_matrix_set_m42(::org::nativescript::canvas::Matrix &matrix, float m42) noexcept;

float org$nativescript$canvas$cxxbridge1$canvas_native_matrix_get_m43(::org::nativescript::canvas::Matrix const &matrix) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_matrix_set_m43(::org::nativescript::canvas::Matrix &matrix, float m43) noexcept;

float org$nativescript$canvas$cxxbridge1$canvas_native_matrix_get_m44(::org::nativescript::canvas::Matrix const &matrix) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_matrix_set_m44(::org::nativescript::canvas::Matrix &matrix, float m44) noexcept;

::org::nativescript::canvas::ImageData *org$nativescript$canvas$cxxbridge1$canvas_native_image_data_create(::std::int32_t width, ::std::int32_t height) noexcept;

::std::int32_t org$nativescript$canvas$cxxbridge1$canvas_native_image_data_get_width(::org::nativescript::canvas::ImageData const &image_data) noexcept;

::std::int32_t org$nativescript$canvas$cxxbridge1$canvas_native_image_data_get_height(::org::nativescript::canvas::ImageData const &image_data) noexcept;

::rust::repr::Fat org$nativescript$canvas$cxxbridge1$canvas_native_image_data_get_data(::org::nativescript::canvas::ImageData &image_data) noexcept;

::org::nativescript::canvas::ImageData *org$nativescript$canvas$cxxbridge1$canvas_native_image_data_get_shared_instance(::org::nativescript::canvas::ImageData &image_data) noexcept;

::org::nativescript::canvas::ImageAsset *org$nativescript$canvas$cxxbridge1$canvas_native_image_asset_create() noexcept;

::org::nativescript::canvas::ImageAsset *org$nativescript$canvas$cxxbridge1$canvas_native_image_asset_shared_clone(::org::nativescript::canvas::ImageAsset const &asset) noexcept;

bool org$nativescript$canvas$cxxbridge1$canvas_native_image_asset_load_from_fd(::org::nativescript::canvas::ImageAsset &asset, ::std::int32_t fd) noexcept;

bool org$nativescript$canvas$cxxbridge1$canvas_native_image_asset_load_from_path(::org::nativescript::canvas::ImageAsset &asset, ::rust::Str path) noexcept;

bool org$nativescript$canvas$cxxbridge1$canvas_native_image_asset_load_from_url(::org::nativescript::canvas::ImageAsset &asset, ::rust::Str url) noexcept;

bool org$nativescript$canvas$cxxbridge1$canvas_native_image_asset_load_from_raw(::org::nativescript::canvas::ImageAsset &asset, ::rust::Slice<::std::uint8_t const> array) noexcept;

::std::int64_t org$nativescript$canvas$cxxbridge1$canvas_native_image_asset_addr(::org::nativescript::canvas::ImageAsset &asset) noexcept;

::std::uint32_t org$nativescript$canvas$cxxbridge1$canvas_native_image_asset_width(::org::nativescript::canvas::ImageAsset &asset) noexcept;

::std::uint32_t org$nativescript$canvas$cxxbridge1$canvas_native_image_asset_height(::org::nativescript::canvas::ImageAsset &asset) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_image_asset_get_error(::org::nativescript::canvas::ImageAsset &asset, ::rust::String *return$) noexcept;

bool org$nativescript$canvas$cxxbridge1$canvas_native_image_asset_has_error(::org::nativescript::canvas::ImageAsset &asset) noexcept;

bool org$nativescript$canvas$cxxbridge1$canvas_native_image_asset_scale(::org::nativescript::canvas::ImageAsset &asset, ::std::uint32_t x, ::std::uint32_t y) noexcept;

bool org$nativescript$canvas$cxxbridge1$canvas_native_image_asset_save_path(::org::nativescript::canvas::ImageAsset &asset, ::rust::Str path, ::std::uint32_t format) noexcept;

float org$nativescript$canvas$cxxbridge1$canvas_native_text_metrics_get_width(::org::nativescript::canvas::TextMetrics const &metrics) noexcept;

float org$nativescript$canvas$cxxbridge1$canvas_native_text_metrics_get_actual_bounding_box_left(::org::nativescript::canvas::TextMetrics const &metrics) noexcept;

float org$nativescript$canvas$cxxbridge1$canvas_native_text_metrics_get_actual_bounding_box_right(::org::nativescript::canvas::TextMetrics const &metrics) noexcept;

float org$nativescript$canvas$cxxbridge1$canvas_native_text_metrics_get_actual_bounding_box_ascent(::org::nativescript::canvas::TextMetrics const &metrics) noexcept;

float org$nativescript$canvas$cxxbridge1$canvas_native_text_metrics_get_actual_bounding_box_descent(::org::nativescript::canvas::TextMetrics const &metrics) noexcept;

float org$nativescript$canvas$cxxbridge1$canvas_native_text_metrics_get_font_bounding_box_ascent(::org::nativescript::canvas::TextMetrics const &metrics) noexcept;

float org$nativescript$canvas$cxxbridge1$canvas_native_text_metrics_get_font_bounding_box_descent(::org::nativescript::canvas::TextMetrics const &metrics) noexcept;

float org$nativescript$canvas$cxxbridge1$canvas_native_text_metrics_get_em_height_ascent(::org::nativescript::canvas::TextMetrics const &metrics) noexcept;

float org$nativescript$canvas$cxxbridge1$canvas_native_text_metrics_get_em_height_descent(::org::nativescript::canvas::TextMetrics const &metrics) noexcept;

float org$nativescript$canvas$cxxbridge1$canvas_native_text_metrics_get_hanging_baseline(::org::nativescript::canvas::TextMetrics const &metrics) noexcept;

float org$nativescript$canvas$cxxbridge1$canvas_native_text_metrics_get_alphabetic_baseline(::org::nativescript::canvas::TextMetrics const &metrics) noexcept;

float org$nativescript$canvas$cxxbridge1$canvas_native_text_metrics_get_ideographic_baseline(::org::nativescript::canvas::TextMetrics const &metrics) noexcept;

::org::nativescript::canvas::PaintStyle *org$nativescript$canvas$cxxbridge1$canvas_native_paint_style_from_bytes(::org::nativescript::canvas::CanvasRenderingContext2D const &context, ::std::int32_t repetition, ::std::int32_t width, ::std::int32_t height, ::rust::Slice<::std::uint8_t const> bytes) noexcept;

::org::nativescript::canvas::PaintStyle *org$nativescript$canvas$cxxbridge1$canvas_native_pattern_from_ptr(::std::int64_t ptr) noexcept;

::org::nativescript::canvas::PaintStyle *org$nativescript$canvas$cxxbridge1$canvas_native_paint_style_empty() noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_gradient_add_color_stop(::org::nativescript::canvas::PaintStyle &style, float stop, ::rust::Str color) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_pattern_set_transform(::org::nativescript::canvas::PaintStyle &pattern, ::org::nativescript::canvas::Matrix const &matrix) noexcept;

::org::nativescript::canvas::TextDecoder *org$nativescript$canvas$cxxbridge1$canvas_native_text_decoder_create(::rust::Str decoding) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_text_decoder_get_encoding(::org::nativescript::canvas::TextDecoder &decoder, ::rust::String *return$) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_text_decoder_decode(::org::nativescript::canvas::TextDecoder &decoder, ::rust::Slice<::std::uint8_t const> data, ::rust::String *return$) noexcept;

::org::nativescript::canvas::TextEncoder *org$nativescript$canvas$cxxbridge1$canvas_native_text_encoder_create(::rust::Str encoding) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_text_encoder_get_encoding(::org::nativescript::canvas::TextEncoder &decoder, ::rust::String *return$) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_text_encoder_encode(::org::nativescript::canvas::TextEncoder &encoder, ::rust::Str text, ::rust::Vec<::std::uint8_t> *return$) noexcept;

bool org$nativescript$canvas$cxxbridge1$canvas_native_context_gl_make_current(::org::nativescript::canvas::CanvasRenderingContext2D const &context) noexcept;

bool org$nativescript$canvas$cxxbridge1$canvas_native_context_gl_swap_buffers(::org::nativescript::canvas::CanvasRenderingContext2D const &context) noexcept;

::org::nativescript::canvas::CanvasRenderingContext2D *org$nativescript$canvas$cxxbridge1$canvas_native_context_create_with_wrapper(::std::int64_t context, ::std::int64_t gl_context) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_context_resize(::org::nativescript::canvas::CanvasRenderingContext2D &context, float width, float height) noexcept;

::org::nativescript::canvas::CanvasRenderingContext2D *org$nativescript$canvas$cxxbridge1$canvas_native_context_create(float width, float height, float density, bool alpha, ::std::int32_t font_color, float ppi, ::std::uint32_t direction) noexcept;

::org::nativescript::canvas::CanvasRenderingContext2D *org$nativescript$canvas$cxxbridge1$canvas_native_context_create_gl(float width, float height, float density, ::std::int64_t gl_context, ::std::int32_t samples, bool alpha, ::std::int32_t font_color, float ppi, ::std::uint32_t direction) noexcept;

::org::nativescript::canvas::CanvasRenderingContext2D *org$nativescript$canvas$cxxbridge1$canvas_native_context_create_with_pointer(::std::int64_t pointer) noexcept;

::org::nativescript::canvas::CanvasRenderingContext2D *org$nativescript$canvas$cxxbridge1$canvas_native_context_create_gl_no_window(float width, float height, float density, ::std::int32_t font_color, float ppi, ::std::uint32_t direction, bool alpha) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_context_flush(::org::nativescript::canvas::CanvasRenderingContext2D const &context) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_context_render(::org::nativescript::canvas::CanvasRenderingContext2D const &context) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_to_data_url(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::rust::Str format, ::std::int32_t quality, ::rust::String *return$) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_to_data_url_string(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::rust::String *format, ::std::int32_t quality, ::rust::String *return$) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_to_data_url_c_string(::org::nativescript::canvas::CanvasRenderingContext2D &context, char const *format, ::std::int32_t quality, ::rust::String *return$) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_context_get_filter(::org::nativescript::canvas::CanvasRenderingContext2D const &context, ::rust::String *return$) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_context_set_filter(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::rust::Str font) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_context_get_font(::org::nativescript::canvas::CanvasRenderingContext2D const &context, ::rust::String *return$) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_context_set_font(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::rust::Str font) noexcept;

float org$nativescript$canvas$cxxbridge1$canvas_native_context_get_global_alpha(::org::nativescript::canvas::CanvasRenderingContext2D const &context) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_context_set_global_alpha(::org::nativescript::canvas::CanvasRenderingContext2D &context, float alpha) noexcept;

bool org$nativescript$canvas$cxxbridge1$canvas_native_context_get_image_smoothing_enabled(::org::nativescript::canvas::CanvasRenderingContext2D const &context) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_context_set_image_smoothing_enabled(::org::nativescript::canvas::CanvasRenderingContext2D &context, bool enabled) noexcept;

::rust::repr::Fat org$nativescript$canvas$cxxbridge1$canvas_native_context_get_image_smoothing_quality(::org::nativescript::canvas::CanvasRenderingContext2D const &context) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_context_set_image_smoothing_quality(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::rust::Str quality) noexcept;

::rust::repr::Fat org$nativescript$canvas$cxxbridge1$canvas_native_context_get_line_join(::org::nativescript::canvas::CanvasRenderingContext2D const &context) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_context_set_line_join(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::rust::Str join) noexcept;

::rust::repr::Fat org$nativescript$canvas$cxxbridge1$canvas_native_context_get_line_cap(::org::nativescript::canvas::CanvasRenderingContext2D const &context) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_context_set_line_cap(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::rust::Str cap) noexcept;

float org$nativescript$canvas$cxxbridge1$canvas_native_context_get_miter_limit(::org::nativescript::canvas::CanvasRenderingContext2D const &context) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_context_set_miter_limit(::org::nativescript::canvas::CanvasRenderingContext2D &context, float limit) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_context_get_shadow_color(::org::nativescript::canvas::CanvasRenderingContext2D const &context, ::rust::String *return$) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_context_get_shadow_color_buf(::org::nativescript::canvas::CanvasRenderingContext2D const &context, ::rust::Vec<::std::uint8_t> *return$) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_context_get_shadow_color_rgba(::org::nativescript::canvas::CanvasRenderingContext2D const &context, ::std::uint8_t &r, ::std::uint8_t &g, ::std::uint8_t &b, ::std::uint8_t &a) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_context_set_shadow_color(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::rust::Str color) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_context_set_shadow_color_rgba(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::std::uint8_t r, ::std::uint8_t g, ::std::uint8_t b, ::std::uint8_t a) noexcept;

float org$nativescript$canvas$cxxbridge1$canvas_native_context_get_shadow_blur(::org::nativescript::canvas::CanvasRenderingContext2D const &context) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_context_set_shadow_blur(::org::nativescript::canvas::CanvasRenderingContext2D &context, float blur) noexcept;

float org$nativescript$canvas$cxxbridge1$canvas_native_context_get_shadow_offset_x(::org::nativescript::canvas::CanvasRenderingContext2D const &context) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_context_set_shadow_offset_x(::org::nativescript::canvas::CanvasRenderingContext2D &context, float x) noexcept;

float org$nativescript$canvas$cxxbridge1$canvas_native_context_get_shadow_offset_y(::org::nativescript::canvas::CanvasRenderingContext2D const &context) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_context_set_shadow_offset_y(::org::nativescript::canvas::CanvasRenderingContext2D &context, float y) noexcept;

::rust::repr::Fat org$nativescript$canvas$cxxbridge1$canvas_native_context_get_text_align(::org::nativescript::canvas::CanvasRenderingContext2D const &context) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_context_set_text_align(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::rust::Str alignment) noexcept;

::rust::repr::Fat org$nativescript$canvas$cxxbridge1$canvas_native_context_get_global_composition(::org::nativescript::canvas::CanvasRenderingContext2D const &context) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_context_set_global_composition(::org::nativescript::canvas::CanvasRenderingContext2D const &context, ::rust::Str composition) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_paint_style_set_fill_color_with_string(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::rust::Str color) noexcept;

bool org$nativescript$canvas$cxxbridge1$canvas_native_parse_css_color_rgba(::rust::Str value, ::std::uint8_t &r, ::std::uint8_t &g, ::std::uint8_t &b, ::std::uint8_t &a) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_paint_style_set_stroke_color_with_rgba(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::std::uint8_t r, ::std::uint8_t g, ::std::uint8_t b, ::std::uint8_t a) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_paint_style_set_fill_color_with_rgba(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::std::uint8_t r, ::std::uint8_t g, ::std::uint8_t b, ::std::uint8_t a) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_paint_style_set_stroke_color_with_string(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::rust::Str color) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_paint_style_get_color_string(::org::nativescript::canvas::PaintStyle &color, ::rust::String *return$) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_paint_style_get_current_stroke_color_string(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::rust::String *return$) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_paint_style_get_current_stroke_color_buf(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::rust::Vec<::std::uint8_t> *return$) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_paint_style_get_current_stroke_color_r_g_b_a(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::std::uint8_t &r, ::std::uint8_t &g, ::std::uint8_t &b, ::std::uint8_t &a) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_paint_style_get_current_fill_color_string(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::rust::String *return$) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_paint_style_get_current_fill_color_buf(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::rust::Vec<::std::uint8_t> *return$) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_paint_style_get_current_fill_color_r_g_b_a(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::std::uint8_t &r, ::std::uint8_t &g, ::std::uint8_t &b, ::std::uint8_t &a) noexcept;

::org::nativescript::canvas::PaintStyleType org$nativescript$canvas$cxxbridge1$canvas_native_context_get_style_type(::org::nativescript::canvas::PaintStyle const &style) noexcept;

::org::nativescript::canvas::PaintStyleType org$nativescript$canvas$cxxbridge1$canvas_native_context_get_current_fill_style_type(::org::nativescript::canvas::CanvasRenderingContext2D &context) noexcept;

::org::nativescript::canvas::PaintStyleType org$nativescript$canvas$cxxbridge1$canvas_native_context_get_current_stroke_style_type(::org::nativescript::canvas::CanvasRenderingContext2D &context) noexcept;

::org::nativescript::canvas::PaintStyle *org$nativescript$canvas$cxxbridge1$canvas_native_context_get_fill_style(::org::nativescript::canvas::CanvasRenderingContext2D &context) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_context_set_fill_style(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::org::nativescript::canvas::PaintStyle const &style) noexcept;

::org::nativescript::canvas::PaintStyle *org$nativescript$canvas$cxxbridge1$canvas_native_context_get_stroke_style(::org::nativescript::canvas::CanvasRenderingContext2D const &context) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_context_set_stroke_style(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::org::nativescript::canvas::PaintStyle const &style) noexcept;

float org$nativescript$canvas$cxxbridge1$canvas_native_context_get_line_width(::org::nativescript::canvas::CanvasRenderingContext2D const &context) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_context_set_line_width(::org::nativescript::canvas::CanvasRenderingContext2D &context, float width) noexcept;

float org$nativescript$canvas$cxxbridge1$canvas_native_context_get_line_dash_offset(::org::nativescript::canvas::CanvasRenderingContext2D const &context) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_context_set_line_dash_offset(::org::nativescript::canvas::CanvasRenderingContext2D &context, float offset) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_context_get_line_dash(::org::nativescript::canvas::CanvasRenderingContext2D const &context, ::rust::Vec<float> *return$) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_context_set_line_dash(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::rust::Slice<float const> dash) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_context_arc(::org::nativescript::canvas::CanvasRenderingContext2D &context, float x, float y, float radius, float start_angle, float end_angle, bool anticlockwise) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_context_arc_to(::org::nativescript::canvas::CanvasRenderingContext2D &context, float x1, float y1, float x2, float y2, float radius) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_context_begin_path(::org::nativescript::canvas::CanvasRenderingContext2D &context) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_context_bezier_curve_to(::org::nativescript::canvas::CanvasRenderingContext2D &context, float cp1x, float cp1y, float cp2x, float cp2y, float x, float y) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_context_clear_rect(::org::nativescript::canvas::CanvasRenderingContext2D &context, float x, float y, float width, float height) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_context_clip(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::org::nativescript::canvas::Path &path, ::rust::Str rule) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_context_clip_rule(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::rust::Str rule) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_context_close_path(::org::nativescript::canvas::CanvasRenderingContext2D &context) noexcept;

::org::nativescript::canvas::ImageData *org$nativescript$canvas$cxxbridge1$canvas_native_context_create_image_data(::std::int32_t width, ::std::int32_t height) noexcept;

::org::nativescript::canvas::ImageData *org$nativescript$canvas$cxxbridge1$canvas_native_context_create_image_data_with_data(::std::int32_t width, ::std::int32_t height, ::rust::Slice<::std::uint8_t const> data) noexcept;

::org::nativescript::canvas::PaintStyle *org$nativescript$canvas$cxxbridge1$canvas_native_context_create_linear_gradient(::org::nativescript::canvas::CanvasRenderingContext2D &context, float x0, float y0, float x1, float y1) noexcept;

::org::nativescript::canvas::PaintStyle *org$nativescript$canvas$cxxbridge1$canvas_native_context_create_pattern(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::rust::Slice<::std::uint8_t const> data, ::std::int32_t width, ::std::int32_t height, ::rust::Str repetition) noexcept;

::org::nativescript::canvas::PaintStyle *org$nativescript$canvas$cxxbridge1$canvas_native_context_create_pattern_asset(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::org::nativescript::canvas::ImageAsset &asset, ::rust::Str repetition) noexcept;

::org::nativescript::canvas::PaintStyle *org$nativescript$canvas$cxxbridge1$canvas_native_context_create_pattern_canvas2d(::org::nativescript::canvas::CanvasRenderingContext2D &source, ::org::nativescript::canvas::CanvasRenderingContext2D &context, ::rust::Str repetition) noexcept;

::org::nativescript::canvas::PaintStyle *org$nativescript$canvas$cxxbridge1$canvas_native_context_create_pattern_encoded(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::rust::Slice<::std::uint8_t const> data, ::rust::Str repetition) noexcept;

::org::nativescript::canvas::PaintStyle *org$nativescript$canvas$cxxbridge1$canvas_native_context_create_radial_gradient(::org::nativescript::canvas::CanvasRenderingContext2D &context, float x0, float y0, float r0, float x1, float y1, float r1) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_context_draw_paint(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::rust::Str color) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_context_draw_point(::org::nativescript::canvas::CanvasRenderingContext2D &context, float x, float y) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_context_draw_points(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::std::int32_t mode, ::rust::Slice<float const> points) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_context_draw_image_dx_dy(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::rust::Slice<::std::uint8_t const> data, float width, float height, float dx, float dy) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_context_draw_image_dx_dy_dw_dh(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::rust::Slice<::std::uint8_t const> data, float width, float height, float dx, float dy, float d_width, float d_height) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_context_draw_image(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::rust::Slice<::std::uint8_t const> data, float width, float height, float sx, float sy, float s_width, float s_height, float dx, float dy, float d_width, float d_height) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_context_draw_image_encoded_dx_dy(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::rust::Slice<::std::uint8_t const> data, float dx, float dy) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_context_draw_image_encoded_dx_dy_dw_dh(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::rust::Slice<::std::uint8_t const> data, float dx, float dy, float d_width, float d_height) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_context_draw_image_encoded(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::rust::Slice<::std::uint8_t const> data, float sx, float sy, float s_width, float s_height, float dx, float dy, float d_width, float d_height) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_context_draw_image_dx_dy_context(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::org::nativescript::canvas::CanvasRenderingContext2D &source, float dx, float dy) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_context_draw_image_dx_dy_dw_dh_context(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::org::nativescript::canvas::CanvasRenderingContext2D &source, float dx, float dy, float d_width, float d_height) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_context_draw_image_context(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::org::nativescript::canvas::CanvasRenderingContext2D &source, float sx, float sy, float s_width, float s_height, float dx, float dy, float d_width, float d_height) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_context_draw_image_dx_dy_asset(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::org::nativescript::canvas::ImageAsset &asset, float dx, float dy) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_context_draw_image_dx_dy_dw_dh_asset(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::org::nativescript::canvas::ImageAsset &asset, float dx, float dy, float d_width, float d_height) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_context_draw_image_asset(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::org::nativescript::canvas::ImageAsset &asset, float sx, float sy, float s_width, float s_height, float dx, float dy, float d_width, float d_height) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_context_ellipse(::org::nativescript::canvas::CanvasRenderingContext2D &context, float x, float y, float radius_x, float radius_y, float rotation, float start_angle, float end_angle, bool anticlockwise) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_context_fill(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::rust::Str rule) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_context_fill_with_path(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::org::nativescript::canvas::Path &path, ::rust::Str rule) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_context_fill_rect(::org::nativescript::canvas::CanvasRenderingContext2D &context, float x, float y, float width, float height) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_context_fill_text(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::rust::Str text, float x, float y, float width) noexcept;

::org::nativescript::canvas::ImageData *org$nativescript$canvas$cxxbridge1$canvas_native_context_get_image_data(::org::nativescript::canvas::CanvasRenderingContext2D &context, float sx, float sy, float sw, float sh) noexcept;

::org::nativescript::canvas::Matrix *org$nativescript$canvas$cxxbridge1$canvas_native_context_get_transform(::org::nativescript::canvas::CanvasRenderingContext2D &context) noexcept;

bool org$nativescript$canvas$cxxbridge1$canvas_native_context_is_point_in_path(::org::nativescript::canvas::CanvasRenderingContext2D &context, float x, float y, ::rust::Str rule) noexcept;

bool org$nativescript$canvas$cxxbridge1$canvas_native_context_is_point_in_path_with_path(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::org::nativescript::canvas::Path &path, float x, float y, ::rust::Str rule) noexcept;

bool org$nativescript$canvas$cxxbridge1$canvas_native_context_is_point_in_stroke(::org::nativescript::canvas::CanvasRenderingContext2D &context, float x, float y) noexcept;

bool org$nativescript$canvas$cxxbridge1$canvas_native_context_is_point_in_stroke_with_path(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::org::nativescript::canvas::Path &path, float x, float y) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_context_line_to(::org::nativescript::canvas::CanvasRenderingContext2D &context, float x, float y) noexcept;

::org::nativescript::canvas::TextMetrics *org$nativescript$canvas$cxxbridge1$canvas_native_context_measure_text(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::rust::Str text) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_context_move_to(::org::nativescript::canvas::CanvasRenderingContext2D &context, float x, float y) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_context_put_image_data(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::org::nativescript::canvas::ImageData &image_data, float dx, float dy, float dirty_x, float dirty_y, float dirty_width, float dirty_height) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_context_quadratic_curve_to(::org::nativescript::canvas::CanvasRenderingContext2D &context, float cpx, float cpy, float x, float y) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_context_rect(::org::nativescript::canvas::CanvasRenderingContext2D &context, float x, float y, float width, float height) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_context_round_rect(::org::nativescript::canvas::CanvasRenderingContext2D &context, float x, float y, float width, float height, ::rust::Slice<float const> radii) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_context_round_rect_tl_tr_br_bl(::org::nativescript::canvas::CanvasRenderingContext2D &context, float x, float y, float width, float height, float top_left, float top_right, float bottom_right, float bottom_left) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_context_reset_transform(::org::nativescript::canvas::CanvasRenderingContext2D &context) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_context_restore(::org::nativescript::canvas::CanvasRenderingContext2D &context) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_context_rotate(::org::nativescript::canvas::CanvasRenderingContext2D &context, float angle) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_context_save(::org::nativescript::canvas::CanvasRenderingContext2D &context) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_context_scale(::org::nativescript::canvas::CanvasRenderingContext2D &context, float x, float y) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_context_set_transform(::org::nativescript::canvas::CanvasRenderingContext2D &context, float a, float b, float c, float d, float e, float f) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_context_set_transform_matrix(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::org::nativescript::canvas::Matrix &matrix) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_context_stroke(::org::nativescript::canvas::CanvasRenderingContext2D &context) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_context_stroke_with_path(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::org::nativescript::canvas::Path &path) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_context_stroke_rect(::org::nativescript::canvas::CanvasRenderingContext2D &context, float x, float y, float width, float height) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_context_stroke_text(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::rust::Str text, float x, float y, float width) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_context_transform(::org::nativescript::canvas::CanvasRenderingContext2D &context, float a, float b, float c, float d, float e, float f) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_context_translate(::org::nativescript::canvas::CanvasRenderingContext2D &context, float x, float y) noexcept;
::std::size_t org$nativescript$canvas$cxxbridge1$WebGLState$operator$sizeof() noexcept;
::std::size_t org$nativescript$canvas$cxxbridge1$WebGLState$operator$alignof() noexcept;
::std::size_t org$nativescript$canvas$cxxbridge1$WebGLActiveInfo$operator$sizeof() noexcept;
::std::size_t org$nativescript$canvas$cxxbridge1$WebGLActiveInfo$operator$alignof() noexcept;
::std::size_t org$nativescript$canvas$cxxbridge1$WebGLResult$operator$sizeof() noexcept;
::std::size_t org$nativescript$canvas$cxxbridge1$WebGLResult$operator$alignof() noexcept;
::std::size_t org$nativescript$canvas$cxxbridge1$ContextAttributes$operator$sizeof() noexcept;
::std::size_t org$nativescript$canvas$cxxbridge1$ContextAttributes$operator$alignof() noexcept;
::std::size_t org$nativescript$canvas$cxxbridge1$WebGLExtension$operator$sizeof() noexcept;
::std::size_t org$nativescript$canvas$cxxbridge1$WebGLExtension$operator$alignof() noexcept;
::std::size_t org$nativescript$canvas$cxxbridge1$WebGLFramebufferAttachmentParameter$operator$sizeof() noexcept;
::std::size_t org$nativescript$canvas$cxxbridge1$WebGLFramebufferAttachmentParameter$operator$alignof() noexcept;
::std::size_t org$nativescript$canvas$cxxbridge1$WebGLShaderPrecisionFormat$operator$sizeof() noexcept;
::std::size_t org$nativescript$canvas$cxxbridge1$WebGLShaderPrecisionFormat$operator$alignof() noexcept;
::std::size_t org$nativescript$canvas$cxxbridge1$EXT_blend_minmax$operator$sizeof() noexcept;
::std::size_t org$nativescript$canvas$cxxbridge1$EXT_blend_minmax$operator$alignof() noexcept;
::std::size_t org$nativescript$canvas$cxxbridge1$EXT_color_buffer_half_float$operator$sizeof() noexcept;
::std::size_t org$nativescript$canvas$cxxbridge1$EXT_color_buffer_half_float$operator$alignof() noexcept;
::std::size_t org$nativescript$canvas$cxxbridge1$EXT_disjoint_timer_query$operator$sizeof() noexcept;
::std::size_t org$nativescript$canvas$cxxbridge1$EXT_disjoint_timer_query$operator$alignof() noexcept;
::std::size_t org$nativescript$canvas$cxxbridge1$EXT_sRGB$operator$sizeof() noexcept;
::std::size_t org$nativescript$canvas$cxxbridge1$EXT_sRGB$operator$alignof() noexcept;
::std::size_t org$nativescript$canvas$cxxbridge1$EXT_shader_texture_lod$operator$sizeof() noexcept;
::std::size_t org$nativescript$canvas$cxxbridge1$EXT_shader_texture_lod$operator$alignof() noexcept;
::std::size_t org$nativescript$canvas$cxxbridge1$EXT_texture_filter_anisotropic$operator$sizeof() noexcept;
::std::size_t org$nativescript$canvas$cxxbridge1$EXT_texture_filter_anisotropic$operator$alignof() noexcept;
::std::size_t org$nativescript$canvas$cxxbridge1$OES_element_index_uint$operator$sizeof() noexcept;
::std::size_t org$nativescript$canvas$cxxbridge1$OES_element_index_uint$operator$alignof() noexcept;
::std::size_t org$nativescript$canvas$cxxbridge1$OES_standard_derivatives$operator$sizeof() noexcept;
::std::size_t org$nativescript$canvas$cxxbridge1$OES_standard_derivatives$operator$alignof() noexcept;
::std::size_t org$nativescript$canvas$cxxbridge1$OES_texture_float$operator$sizeof() noexcept;
::std::size_t org$nativescript$canvas$cxxbridge1$OES_texture_float$operator$alignof() noexcept;
::std::size_t org$nativescript$canvas$cxxbridge1$OES_texture_float_linear$operator$sizeof() noexcept;
::std::size_t org$nativescript$canvas$cxxbridge1$OES_texture_float_linear$operator$alignof() noexcept;
::std::size_t org$nativescript$canvas$cxxbridge1$OES_texture_half_float$operator$sizeof() noexcept;
::std::size_t org$nativescript$canvas$cxxbridge1$OES_texture_half_float$operator$alignof() noexcept;
::std::size_t org$nativescript$canvas$cxxbridge1$OES_texture_half_float_linear$operator$sizeof() noexcept;
::std::size_t org$nativescript$canvas$cxxbridge1$OES_texture_half_float_linear$operator$alignof() noexcept;
::std::size_t org$nativescript$canvas$cxxbridge1$OES_vertex_array_object$operator$sizeof() noexcept;
::std::size_t org$nativescript$canvas$cxxbridge1$OES_vertex_array_object$operator$alignof() noexcept;
::std::size_t org$nativescript$canvas$cxxbridge1$WEBGL_color_buffer_float$operator$sizeof() noexcept;
::std::size_t org$nativescript$canvas$cxxbridge1$WEBGL_color_buffer_float$operator$alignof() noexcept;
::std::size_t org$nativescript$canvas$cxxbridge1$WEBGL_compressed_texture_atc$operator$sizeof() noexcept;
::std::size_t org$nativescript$canvas$cxxbridge1$WEBGL_compressed_texture_atc$operator$alignof() noexcept;
::std::size_t org$nativescript$canvas$cxxbridge1$WEBGL_compressed_texture_etc1$operator$sizeof() noexcept;
::std::size_t org$nativescript$canvas$cxxbridge1$WEBGL_compressed_texture_etc1$operator$alignof() noexcept;
::std::size_t org$nativescript$canvas$cxxbridge1$WEBGL_compressed_texture_s3tc$operator$sizeof() noexcept;
::std::size_t org$nativescript$canvas$cxxbridge1$WEBGL_compressed_texture_s3tc$operator$alignof() noexcept;
::std::size_t org$nativescript$canvas$cxxbridge1$WEBGL_compressed_texture_s3tc_srgb$operator$sizeof() noexcept;
::std::size_t org$nativescript$canvas$cxxbridge1$WEBGL_compressed_texture_s3tc_srgb$operator$alignof() noexcept;
::std::size_t org$nativescript$canvas$cxxbridge1$WEBGL_compressed_texture_etc$operator$sizeof() noexcept;
::std::size_t org$nativescript$canvas$cxxbridge1$WEBGL_compressed_texture_etc$operator$alignof() noexcept;
::std::size_t org$nativescript$canvas$cxxbridge1$WEBGL_compressed_texture_pvrtc$operator$sizeof() noexcept;
::std::size_t org$nativescript$canvas$cxxbridge1$WEBGL_compressed_texture_pvrtc$operator$alignof() noexcept;
::std::size_t org$nativescript$canvas$cxxbridge1$WEBGL_lose_context$operator$sizeof() noexcept;
::std::size_t org$nativescript$canvas$cxxbridge1$WEBGL_lose_context$operator$alignof() noexcept;
::std::size_t org$nativescript$canvas$cxxbridge1$ANGLE_instanced_arrays$operator$sizeof() noexcept;
::std::size_t org$nativescript$canvas$cxxbridge1$ANGLE_instanced_arrays$operator$alignof() noexcept;
::std::size_t org$nativescript$canvas$cxxbridge1$WEBGL_depth_texture$operator$sizeof() noexcept;
::std::size_t org$nativescript$canvas$cxxbridge1$WEBGL_depth_texture$operator$alignof() noexcept;
::std::size_t org$nativescript$canvas$cxxbridge1$WEBGL_draw_buffers$operator$sizeof() noexcept;
::std::size_t org$nativescript$canvas$cxxbridge1$WEBGL_draw_buffers$operator$alignof() noexcept;

::org::nativescript::canvas::PaintStyle *org$nativescript$canvas$cxxbridge1$canvas_native_context_create_pattern_webgl(::org::nativescript::canvas::WebGLState &source, ::org::nativescript::canvas::CanvasRenderingContext2D &context, ::rust::Str repetition) noexcept;

bool org$nativescript$canvas$cxxbridge1$canvas_native_webgl_make_current(::org::nativescript::canvas::WebGLState &state) noexcept;

bool org$nativescript$canvas$cxxbridge1$canvas_native_webgl_swap_buffers(::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_to_data_url(::org::nativescript::canvas::WebGLState &state, ::rust::Str format, ::std::int32_t quality, ::rust::String *return$) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_resized(::org::nativescript::canvas::WebGLState &state) noexcept;

::rust::repr::Fat org$nativescript$canvas$cxxbridge1$canvas_native_webgl_active_info_get_name(::org::nativescript::canvas::WebGLActiveInfo const &info) noexcept;

::std::int32_t org$nativescript$canvas$cxxbridge1$canvas_native_webgl_active_info_get_size(::org::nativescript::canvas::WebGLActiveInfo const &info) noexcept;

::std::uint32_t org$nativescript$canvas$cxxbridge1$canvas_native_webgl_active_info_get_type(::org::nativescript::canvas::WebGLActiveInfo const &info) noexcept;

bool org$nativescript$canvas$cxxbridge1$canvas_native_webgl_active_info_get_is_empty(::org::nativescript::canvas::WebGLActiveInfo const &info) noexcept;

::std::int32_t org$nativescript$canvas$cxxbridge1$canvas_native_webgl_shader_precision_format_get_range_min(::org::nativescript::canvas::WebGLShaderPrecisionFormat const &shader) noexcept;

::std::int32_t org$nativescript$canvas$cxxbridge1$canvas_native_webgl_shader_precision_format_get_range_max(::org::nativescript::canvas::WebGLShaderPrecisionFormat const &shader) noexcept;

::std::int32_t org$nativescript$canvas$cxxbridge1$canvas_native_webgl_shader_precision_format_get_precision(::org::nativescript::canvas::WebGLShaderPrecisionFormat const &shader) noexcept;

bool org$nativescript$canvas$cxxbridge1$canvas_native_webgl_context_attribute_get_get_alpha(::org::nativescript::canvas::ContextAttributes const &attr) noexcept;

bool org$nativescript$canvas$cxxbridge1$canvas_native_webgl_context_attribute_get_get_antialias(::org::nativescript::canvas::ContextAttributes const &attr) noexcept;

bool org$nativescript$canvas$cxxbridge1$canvas_native_webgl_context_attribute_get_get_depth(::org::nativescript::canvas::ContextAttributes const &attr) noexcept;

bool org$nativescript$canvas$cxxbridge1$canvas_native_webgl_context_attribute_get_get_fail_if_major_performance_caveat(::org::nativescript::canvas::ContextAttributes const &attr) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_context_attribute_get_get_power_preference(::org::nativescript::canvas::ContextAttributes const &attr, ::rust::String *return$) noexcept;

bool org$nativescript$canvas$cxxbridge1$canvas_native_webgl_context_attribute_get_get_premultiplied_alpha(::org::nativescript::canvas::ContextAttributes const &attr) noexcept;

bool org$nativescript$canvas$cxxbridge1$canvas_native_webgl_context_attribute_get_get_preserve_drawing_buffer(::org::nativescript::canvas::ContextAttributes const &attr) noexcept;

bool org$nativescript$canvas$cxxbridge1$canvas_native_webgl_context_attribute_get_get_stencil(::org::nativescript::canvas::ContextAttributes const &attr) noexcept;

bool org$nativescript$canvas$cxxbridge1$canvas_native_webgl_context_attribute_get_get_desynchronized(::org::nativescript::canvas::ContextAttributes const &attr) noexcept;

bool org$nativescript$canvas$cxxbridge1$canvas_native_webgl_context_attribute_get_get_xr_compatible(::org::nativescript::canvas::ContextAttributes const &attr) noexcept;

bool org$nativescript$canvas$cxxbridge1$canvas_native_webgl_context_extension_is_none(::org::nativescript::canvas::WebGLExtension const &extension) noexcept;

::org::nativescript::canvas::WebGLExtensionType org$nativescript$canvas$cxxbridge1$canvas_native_webgl_context_extension_get_type(::org::nativescript::canvas::WebGLExtension const &extension) noexcept;

::org::nativescript::canvas::EXT_disjoint_timer_query *org$nativescript$canvas$cxxbridge1$canvas_native_webgl_context_extension_to_ext_disjoint_timer_query(::org::nativescript::canvas::WebGLExtension *extension) noexcept;

::org::nativescript::canvas::ANGLE_instanced_arrays *org$nativescript$canvas$cxxbridge1$canvas_native_webgl_context_extension_to_angle_instanced_arrays(::org::nativescript::canvas::WebGLExtension *extension) noexcept;

::org::nativescript::canvas::WEBGL_lose_context *org$nativescript$canvas$cxxbridge1$canvas_native_webgl_context_extension_to_lose_context(::org::nativescript::canvas::WebGLExtension *extension) noexcept;

::org::nativescript::canvas::WEBGL_draw_buffers *org$nativescript$canvas$cxxbridge1$canvas_native_webgl_context_extension_to_draw_buffers(::org::nativescript::canvas::WebGLExtension *extension) noexcept;

::org::nativescript::canvas::OES_vertex_array_object *org$nativescript$canvas$cxxbridge1$canvas_native_webgl_context_extension_to_oes_vertex_array_object(::org::nativescript::canvas::WebGLExtension *extension) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_lose_context_lose_context(::org::nativescript::canvas::WEBGL_lose_context const &context) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_lose_context_restore_context(::org::nativescript::canvas::WEBGL_lose_context const &context) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_draw_buffers_draw_buffers_webgl(::rust::Slice<::std::uint32_t const> buffers, ::org::nativescript::canvas::WEBGL_draw_buffers const &context) noexcept;

::std::uint32_t org$nativescript$canvas$cxxbridge1$canvas_native_webgl_oes_vertex_array_object_create_vertex_array_oes(::org::nativescript::canvas::OES_vertex_array_object const &object) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_oes_vertex_array_object_delete_vertex_array_oes(::std::uint32_t array_object, ::org::nativescript::canvas::OES_vertex_array_object const &object) noexcept;

bool org$nativescript$canvas$cxxbridge1$canvas_native_webgl_oes_vertex_array_object_is_vertex_array_oes(::std::uint32_t array_object, ::org::nativescript::canvas::OES_vertex_array_object const &object) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_oes_vertex_array_object_bind_vertex_array_oes(::std::uint32_t array_object, ::org::nativescript::canvas::OES_vertex_array_object const &object) noexcept;

bool org$nativescript$canvas$cxxbridge1$canvas_native_webgl_framebuffer_attachment_parameter_get_is_texture(::org::nativescript::canvas::WebGLFramebufferAttachmentParameter const &param) noexcept;

bool org$nativescript$canvas$cxxbridge1$canvas_native_webgl_framebuffer_attachment_parameter_get_is_renderbuffer(::org::nativescript::canvas::WebGLFramebufferAttachmentParameter const &param) noexcept;

::std::int32_t org$nativescript$canvas$cxxbridge1$canvas_native_webgl_framebuffer_attachment_parameter_get_value(::org::nativescript::canvas::WebGLFramebufferAttachmentParameter const &param) noexcept;

::org::nativescript::canvas::WebGLResultType org$nativescript$canvas$cxxbridge1$canvas_native_webgl_result_get_type(::org::nativescript::canvas::WebGLResult const &result) noexcept;

bool org$nativescript$canvas$cxxbridge1$canvas_native_webgl_result_get_bool(::org::nativescript::canvas::WebGLResult const &result) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_result_get_i32_array(::org::nativescript::canvas::WebGLResult const &result, ::rust::Vec<::std::int32_t> *return$) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_result_get_u32_array(::org::nativescript::canvas::WebGLResult const &result, ::rust::Vec<::std::uint32_t> *return$) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_result_get_f32_array(::org::nativescript::canvas::WebGLResult const &result, ::rust::Vec<float> *return$) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_result_get_bool_array(::org::nativescript::canvas::WebGLResult const &result, ::rust::Vec<::std::uint8_t> *return$) noexcept;

::std::uint32_t org$nativescript$canvas$cxxbridge1$canvas_native_webgl_result_get_u32(::org::nativescript::canvas::WebGLResult const &result) noexcept;

::std::int32_t org$nativescript$canvas$cxxbridge1$canvas_native_webgl_result_get_i32(::org::nativescript::canvas::WebGLResult const &result) noexcept;

float org$nativescript$canvas$cxxbridge1$canvas_native_webgl_result_get_f32(::org::nativescript::canvas::WebGLResult const &result) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_result_get_string(::org::nativescript::canvas::WebGLResult const &result, ::rust::String *return$) noexcept;

bool org$nativescript$canvas$cxxbridge1$canvas_native_webgl_result_get_is_none(::org::nativescript::canvas::WebGLResult const &result) noexcept;

::std::int32_t org$nativescript$canvas$cxxbridge1$canvas_native_webgl_state_get_unpack_colorspace_conversion_webgl(::org::nativescript::canvas::WebGLState &state) noexcept;

bool org$nativescript$canvas$cxxbridge1$canvas_native_webgl_state_get_flip_y(::org::nativescript::canvas::WebGLState &state) noexcept;

bool org$nativescript$canvas$cxxbridge1$canvas_native_webgl_state_get_premultiplied_alpha(::org::nativescript::canvas::WebGLState &state) noexcept;

::std::int32_t org$nativescript$canvas$cxxbridge1$canvas_native_webgl_state_get_drawing_buffer_width(::org::nativescript::canvas::WebGLState &state) noexcept;

::std::int32_t org$nativescript$canvas$cxxbridge1$canvas_native_webgl_state_get_drawing_buffer_height(::org::nativescript::canvas::WebGLState &state) noexcept;

::std::uint32_t org$nativescript$canvas$cxxbridge1$canvas_native_webgl_ext_disjoint_timer_query_create_query_ext(::org::nativescript::canvas::EXT_disjoint_timer_query const &query) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_ext_disjoint_timer_query_delete_query_ext(::std::uint32_t value, ::org::nativescript::canvas::EXT_disjoint_timer_query const &query) noexcept;

bool org$nativescript$canvas$cxxbridge1$canvas_native_webgl_ext_disjoint_timer_query_is_query_ext(::std::uint32_t value, ::org::nativescript::canvas::EXT_disjoint_timer_query const &query) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_ext_disjoint_timer_query_begin_query_ext(::std::uint32_t target, ::std::uint32_t value, ::org::nativescript::canvas::EXT_disjoint_timer_query const &query) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_ext_disjoint_timer_query_end_query_ext(::std::uint32_t target, ::org::nativescript::canvas::EXT_disjoint_timer_query const &query) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_ext_disjoint_timer_query_query_counter_ext(::std::uint32_t value, ::std::uint32_t target, ::org::nativescript::canvas::EXT_disjoint_timer_query const &query) noexcept;

::std::int32_t org$nativescript$canvas$cxxbridge1$canvas_native_webgl_ext_disjoint_timer_query_get_query_ext(::std::uint32_t target, ::std::uint32_t pname, ::org::nativescript::canvas::EXT_disjoint_timer_query const &query) noexcept;

::org::nativescript::canvas::WebGLResult *org$nativescript$canvas$cxxbridge1$canvas_native_webgl_ext_disjoint_timer_query_get_query_object_ext(::std::uint32_t target, ::std::uint32_t pname, ::org::nativescript::canvas::EXT_disjoint_timer_query const &query) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_angle_instanced_arrays_draw_arrays_instanced_angle(::std::uint32_t mode, ::std::int32_t first, ::std::int32_t count, ::std::int32_t primcount, ::org::nativescript::canvas::ANGLE_instanced_arrays const &arrays) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_angle_instanced_arrays_draw_elements_instanced_angle(::std::uint32_t mode, ::std::int32_t count, ::std::uint32_t type_, ::std::int32_t offset, ::std::int32_t primcount, ::org::nativescript::canvas::ANGLE_instanced_arrays const &arrays) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_angle_instanced_arrays_vertex_attrib_divisor_angle(::std::uint32_t index, ::std::uint32_t divisor, ::org::nativescript::canvas::ANGLE_instanced_arrays const &arrays) noexcept;

::org::nativescript::canvas::WebGLState *org$nativescript$canvas$cxxbridge1$canvas_native_webgl_create(::std::int64_t gl_context, ::rust::Str version, bool alpha, bool antialias, bool depth, bool fail_if_major_performance_caveat, ::rust::Str power_preference, bool premultiplied_alpha, bool preserve_drawing_buffer, bool stencil, bool desynchronized, bool xr_compatible) noexcept;

::org::nativescript::canvas::WebGLState *org$nativescript$canvas$cxxbridge1$canvas_native_webgl_create_no_window(::std::int32_t width, ::std::int32_t height, ::rust::Str version, bool alpha, bool antialias, bool depth, bool fail_if_major_performance_caveat, ::rust::Str power_preference, bool premultiplied_alpha, bool preserve_drawing_buffer, bool stencil, bool desynchronized, bool xr_compatible, bool is_canvas) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_active_texture(::std::uint32_t texture, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_attach_shader(::std::uint32_t program, ::std::uint32_t shader, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_bind_attrib_location(::std::uint32_t program, ::std::uint32_t index, ::rust::Str name, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_bind_buffer(::std::uint32_t target, ::std::uint32_t buffer, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_bind_frame_buffer(::std::uint32_t target, ::std::uint32_t framebuffer, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_bind_render_buffer(::std::uint32_t target, ::std::uint32_t renderbuffer, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_bind_texture(::std::uint32_t target, ::std::uint32_t texture, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_blend_color(float red, float green, float blue, float alpha, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_blend_equation_separate(::std::uint32_t mode_rgb, ::std::uint32_t mode_alpha, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_blend_equation(::std::uint32_t mode, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_blend_func_separate(::std::uint32_t src_rgb, ::std::uint32_t dst_rgb, ::std::uint32_t src_alpha, ::std::uint32_t dst_alpha, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_blend_func(::std::uint32_t sfactor, ::std::uint32_t dfactor, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_buffer_data(::std::uint32_t target, ::rust::Slice<::std::uint8_t const> src_data, ::std::uint32_t usage, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_buffer_data_u16(::std::uint32_t target, ::rust::Slice<::std::uint16_t const> src_data, ::std::uint32_t usage, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_buffer_data_f32(::std::uint32_t target, ::rust::Slice<float const> src_data, ::std::uint32_t usage, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_buffer_data_none(::std::uint32_t target, ::rust::isize size, ::std::uint32_t usage, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_buffer_sub_data(::std::uint32_t target, ::rust::isize offset, ::rust::Slice<::std::uint8_t const> src_data, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_buffer_sub_data_none(::std::uint32_t target, ::rust::isize offset, ::org::nativescript::canvas::WebGLState &state) noexcept;

::std::uint32_t org$nativescript$canvas$cxxbridge1$canvas_native_webgl_check_frame_buffer_status(::std::uint32_t target, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_clear(::std::uint32_t mask, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_clear_color(float red, float green, float blue, float alpha, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_clear_depth(float depth, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_clear_stencil(::std::int32_t stencil, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_color_mask(bool red, bool green, bool blue, bool alpha, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_commit(::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_compile_shader(::std::uint32_t shader, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_compressed_tex_image2d(::std::uint32_t target, ::std::int32_t level, ::std::uint32_t internalformat, ::std::int32_t width, ::std::int32_t height, ::std::int32_t border, ::rust::Slice<::std::uint8_t const> pixels, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_compressed_tex_image2d_none(::std::uint32_t target, ::std::int32_t level, ::std::uint32_t internalformat, ::std::int32_t width, ::std::int32_t height, ::std::int32_t border, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_compressed_tex_sub_image2d(::std::uint32_t target, ::std::int32_t level, ::std::int32_t xoffset, ::std::int32_t yoffset, ::std::int32_t width, ::std::int32_t height, ::std::uint32_t format, ::rust::Slice<::std::uint8_t const> pixels, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_copy_tex_image2d(::std::uint32_t target, ::std::int32_t level, ::std::uint32_t internalformat, ::std::int32_t x, ::std::int32_t y, ::std::int32_t width, ::std::int32_t height, ::std::int32_t border, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_copy_tex_sub_image2d(::std::uint32_t target, ::std::int32_t level, ::std::int32_t xoffset, ::std::int32_t yoffset, ::std::int32_t x, ::std::int32_t y, ::std::int32_t width, ::std::int32_t height, ::org::nativescript::canvas::WebGLState &state) noexcept;

::std::uint32_t org$nativescript$canvas$cxxbridge1$canvas_native_webgl_create_buffer(::org::nativescript::canvas::WebGLState &state) noexcept;

::std::uint32_t org$nativescript$canvas$cxxbridge1$canvas_native_webgl_create_framebuffer(::org::nativescript::canvas::WebGLState &state) noexcept;

::std::uint32_t org$nativescript$canvas$cxxbridge1$canvas_native_webgl_create_program(::org::nativescript::canvas::WebGLState &state) noexcept;

::std::uint32_t org$nativescript$canvas$cxxbridge1$canvas_native_webgl_create_renderbuffer(::org::nativescript::canvas::WebGLState &state) noexcept;

::std::uint32_t org$nativescript$canvas$cxxbridge1$canvas_native_webgl_create_shader(::std::uint32_t shader_type, ::org::nativescript::canvas::WebGLState &state) noexcept;

::std::uint32_t org$nativescript$canvas$cxxbridge1$canvas_native_webgl_create_texture(::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_cull_face(::std::uint32_t mode, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_delete_buffer(::std::uint32_t buffer, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_delete_framebuffer(::std::uint32_t frame_buffer, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_delete_program(::std::uint32_t program, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_delete_renderbuffer(::std::uint32_t render_buffer, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_delete_shader(::std::uint32_t shader, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_delete_texture(::std::uint32_t texture, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_depth_func(::std::uint32_t func, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_depth_mask(bool flag, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_depth_range(float z_near, float z_far, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_detach_shader(::std::uint32_t program, ::std::uint32_t shader, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_disable(::std::uint32_t cap, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_disable_vertex_attrib_array(::std::uint32_t index, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_draw_arrays(::std::uint32_t mode, ::std::int32_t first, ::std::int32_t count, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_draw_elements(::std::uint32_t mode, ::std::int32_t count, ::std::uint32_t element_type, ::rust::isize offset, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_enable(::std::uint32_t cap, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_enable_vertex_attrib_array(::std::uint32_t index, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_finish(::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_flush(::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_framebuffer_renderbuffer(::std::uint32_t target, ::std::uint32_t attachment, ::std::uint32_t renderbuffertarget, ::std::uint32_t renderbuffer, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_framebuffer_texture2d(::std::uint32_t target, ::std::uint32_t attachment, ::std::uint32_t textarget, ::std::uint32_t texture, ::std::int32_t level, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_front_face(::std::uint32_t mode, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_generate_mipmap(::std::uint32_t target, ::org::nativescript::canvas::WebGLState &state) noexcept;

::org::nativescript::canvas::WebGLActiveInfo *org$nativescript$canvas$cxxbridge1$canvas_native_webgl_get_active_attrib(::std::uint32_t program, ::std::uint32_t index, ::org::nativescript::canvas::WebGLState &state) noexcept;

::org::nativescript::canvas::WebGLActiveInfo *org$nativescript$canvas$cxxbridge1$canvas_native_webgl_get_active_uniform(::std::uint32_t program, ::std::uint32_t index, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_get_attached_shaders(::std::uint32_t program, ::org::nativescript::canvas::WebGLState &state, ::rust::Vec<::std::uint32_t> *return$) noexcept;

::std::int32_t org$nativescript$canvas$cxxbridge1$canvas_native_webgl_get_attrib_location(::std::uint32_t program, ::rust::Str name, ::org::nativescript::canvas::WebGLState &state) noexcept;

::std::int32_t org$nativescript$canvas$cxxbridge1$canvas_native_webgl_get_buffer_parameter(::std::uint32_t target, ::std::uint32_t pname, ::org::nativescript::canvas::WebGLState &state) noexcept;

::org::nativescript::canvas::ContextAttributes *org$nativescript$canvas$cxxbridge1$canvas_native_webgl_get_context_attributes(::org::nativescript::canvas::WebGLState &state) noexcept;

::std::uint32_t org$nativescript$canvas$cxxbridge1$canvas_native_webgl_get_error(::org::nativescript::canvas::WebGLState &state) noexcept;

::org::nativescript::canvas::WebGLExtension *org$nativescript$canvas$cxxbridge1$canvas_native_webgl_get_extension(::rust::Str name, ::org::nativescript::canvas::WebGLState &state) noexcept;

::org::nativescript::canvas::WebGLFramebufferAttachmentParameter *org$nativescript$canvas$cxxbridge1$canvas_native_webgl_get_framebuffer_attachment_parameter(::std::uint32_t target, ::std::uint32_t attachment, ::std::uint32_t pname, ::org::nativescript::canvas::WebGLState &state) noexcept;

::org::nativescript::canvas::WebGLResult *org$nativescript$canvas$cxxbridge1$canvas_native_webgl_get_parameter(::std::uint32_t pname, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_get_program_info_log(::std::uint32_t program, ::org::nativescript::canvas::WebGLState &state, ::rust::String *return$) noexcept;

::org::nativescript::canvas::WebGLResult *org$nativescript$canvas$cxxbridge1$canvas_native_webgl_get_program_parameter(::std::uint32_t program, ::std::uint32_t pname, ::org::nativescript::canvas::WebGLState &state) noexcept;

::std::int32_t org$nativescript$canvas$cxxbridge1$canvas_native_webgl_get_renderbuffer_parameter(::std::uint32_t target, ::std::uint32_t pname, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_get_shader_info_log(::std::uint32_t shader, ::org::nativescript::canvas::WebGLState &state, ::rust::String *return$) noexcept;

::org::nativescript::canvas::WebGLResult *org$nativescript$canvas$cxxbridge1$canvas_native_webgl_get_shader_parameter(::std::uint32_t shader, ::std::uint32_t pname, ::org::nativescript::canvas::WebGLState &state) noexcept;

::org::nativescript::canvas::WebGLShaderPrecisionFormat *org$nativescript$canvas$cxxbridge1$canvas_native_webgl_get_shader_precision_format(::std::uint32_t shader_type, ::std::uint32_t precision_type, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_get_shader_source(::std::uint32_t shader, ::org::nativescript::canvas::WebGLState &state, ::rust::String *return$) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_get_supported_extensions(::org::nativescript::canvas::WebGLState &state, ::rust::Vec<::rust::String> *return$) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_get_supported_extensions_to_string(::org::nativescript::canvas::WebGLState &state, ::rust::String *return$) noexcept;

::std::int32_t org$nativescript$canvas$cxxbridge1$canvas_native_webgl_get_tex_parameter(::std::uint32_t target, ::std::uint32_t pname, ::org::nativescript::canvas::WebGLState &state) noexcept;

::std::int32_t org$nativescript$canvas$cxxbridge1$canvas_native_webgl_get_uniform_location(::std::uint32_t program, ::rust::Str name, ::org::nativescript::canvas::WebGLState &state) noexcept;

::org::nativescript::canvas::WebGLResult *org$nativescript$canvas$cxxbridge1$canvas_native_webgl_get_uniform(::std::uint32_t program, ::std::int32_t location, ::org::nativescript::canvas::WebGLState &state) noexcept;

::std::size_t org$nativescript$canvas$cxxbridge1$canvas_native_webgl_get_vertex_attrib_offset(::std::uint32_t index, ::std::uint32_t pname, ::org::nativescript::canvas::WebGLState &state) noexcept;

::org::nativescript::canvas::WebGLResult *org$nativescript$canvas$cxxbridge1$canvas_native_webgl_get_vertex_attrib(::std::uint32_t index, ::std::uint32_t pname, ::org::nativescript::canvas::WebGLState &state) noexcept;

bool org$nativescript$canvas$cxxbridge1$canvas_native_webgl_get_is_context_lost(::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_hint(::std::uint32_t target, ::std::uint32_t mode, ::org::nativescript::canvas::WebGLState &state) noexcept;

bool org$nativescript$canvas$cxxbridge1$canvas_native_webgl_is_buffer(::std::uint32_t buffer, ::org::nativescript::canvas::WebGLState &state) noexcept;

bool org$nativescript$canvas$cxxbridge1$canvas_native_webgl_is_enabled(::std::uint32_t cap, ::org::nativescript::canvas::WebGLState &state) noexcept;

bool org$nativescript$canvas$cxxbridge1$canvas_native_webgl_is_framebuffer(::std::uint32_t framebuffer, ::org::nativescript::canvas::WebGLState &state) noexcept;

bool org$nativescript$canvas$cxxbridge1$canvas_native_webgl_is_program(::std::uint32_t program, ::org::nativescript::canvas::WebGLState &state) noexcept;

bool org$nativescript$canvas$cxxbridge1$canvas_native_webgl_is_renderbuffer(::std::uint32_t renderbuffer, ::org::nativescript::canvas::WebGLState &state) noexcept;

bool org$nativescript$canvas$cxxbridge1$canvas_native_webgl_is_shader(::std::uint32_t shader, ::org::nativescript::canvas::WebGLState &state) noexcept;

bool org$nativescript$canvas$cxxbridge1$canvas_native_webgl_is_texture(::std::uint32_t texture, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_line_width(float width, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_link_program(::std::uint32_t program, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_pixel_storei(::std::uint32_t pname, ::std::int32_t param, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_polygon_offset(float factor, float units, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_read_pixels_u8(::std::int32_t x, ::std::int32_t y, ::std::int32_t width, ::std::int32_t height, ::std::uint32_t format, ::std::uint32_t pixel_type, ::rust::Slice<::std::uint8_t > pixels, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_read_pixels_u16(::std::int32_t x, ::std::int32_t y, ::std::int32_t width, ::std::int32_t height, ::std::uint32_t format, ::std::uint32_t pixel_type, ::rust::Slice<::std::uint16_t > pixels, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_read_pixels_f32(::std::int32_t x, ::std::int32_t y, ::std::int32_t width, ::std::int32_t height, ::std::uint32_t format, ::std::uint32_t pixel_type, ::rust::Slice<float > pixels, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_renderbuffer_storage(::std::uint32_t target, ::std::uint32_t internal_format, ::std::int32_t width, ::std::int32_t height, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_sample_coverage(float value, bool invert, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_scissor(::std::int32_t x, ::std::int32_t y, ::std::int32_t width, ::std::int32_t height, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_shader_source(::std::uint32_t shader, ::rust::Str source, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_stencil_func(::std::uint32_t func, ::std::int32_t reference, ::std::uint32_t mask, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_stencil_func_separate(::std::uint32_t face, ::std::uint32_t func, ::std::int32_t reference, ::std::uint32_t mask, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_stencil_mask(::std::uint32_t mask, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_stencil_mask_separate(::std::uint32_t face, ::std::uint32_t mask, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_stencil_op(::std::uint32_t fail, ::std::uint32_t zfail, ::std::uint32_t zpass, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_stencil_op_separate(::std::uint32_t face, ::std::uint32_t fail, ::std::uint32_t zfail, ::std::uint32_t zpass, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_tex_image2d_image_none(::std::int32_t target, ::std::int32_t level, ::std::int32_t internalformat, ::std::int32_t format, ::std::int32_t image_type, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_tex_image2d_canvas2d(::std::int32_t target, ::std::int32_t level, ::std::int32_t internalformat, ::std::int32_t format, ::std::int32_t image_type, ::org::nativescript::canvas::CanvasRenderingContext2D &canvas, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_tex_image2d_webgl(::std::int32_t target, ::std::int32_t level, ::std::int32_t internalformat, ::std::int32_t format, ::std::int32_t image_type, ::org::nativescript::canvas::WebGLState &webgl, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_tex_image2d(::std::int32_t target, ::std::int32_t level, ::std::int32_t internalformat, ::std::int32_t width, ::std::int32_t height, ::std::int32_t border, ::std::int32_t format, ::std::int32_t image_type, ::rust::Slice<::std::uint8_t > buf, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_tex_image2d_none(::std::int32_t target, ::std::int32_t level, ::std::int32_t internalformat, ::std::int32_t width, ::std::int32_t height, ::std::int32_t border, ::std::int32_t format, ::std::int32_t image_type, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_tex_image2d_image_asset(::std::int32_t target, ::std::int32_t level, ::std::int32_t internalformat, ::std::int32_t format, ::std::int32_t image_type, ::org::nativescript::canvas::ImageAsset &image_asset, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_tex_parameterf(::std::uint32_t target, ::std::uint32_t pname, float param, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_tex_parameteri(::std::uint32_t target, ::std::uint32_t pname, ::std::int32_t param, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_tex_sub_image2d_asset(::std::uint32_t target, ::std::int32_t level, ::std::int32_t xoffset, ::std::int32_t yoffset, ::std::uint32_t format, ::std::int32_t image_type, ::org::nativescript::canvas::ImageAsset &asset, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_tex_sub_image2d_canvas2d(::std::uint32_t target, ::std::int32_t level, ::std::int32_t xoffset, ::std::int32_t yoffset, ::std::uint32_t format, ::std::int32_t image_type, ::org::nativescript::canvas::CanvasRenderingContext2D &canvas, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_tex_sub_image2d_webgl(::std::uint32_t target, ::std::int32_t level, ::std::int32_t xoffset, ::std::int32_t yoffset, ::std::uint32_t format, ::std::int32_t image_type, ::org::nativescript::canvas::WebGLState &webgl, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_tex_sub_image2d(::std::uint32_t target, ::std::int32_t level, ::std::int32_t xoffset, ::std::int32_t yoffset, ::std::int32_t width, ::std::int32_t height, ::std::uint32_t format, ::std::int32_t image_type, ::rust::Slice<::std::uint8_t const> buf, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_uniform1f(::std::int32_t location, float v0, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_uniform1fv(::std::int32_t location, ::rust::Slice<float const> value, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_uniform1i(::std::int32_t location, ::std::int32_t v0, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_uniform1iv(::std::int32_t location, ::rust::Slice<::std::int32_t const> value, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_uniform2f(::std::int32_t location, float v0, float v1, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_uniform2fv(::std::int32_t location, ::rust::Slice<float const> value, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_uniform2i(::std::int32_t location, ::std::int32_t v0, ::std::int32_t v1, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_uniform2iv(::std::int32_t location, ::rust::Slice<::std::int32_t const> value, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_uniform3f(::std::int32_t location, float v0, float v1, float v2, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_uniform3fv(::std::int32_t location, ::rust::Slice<float const> value, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_uniform3i(::std::int32_t location, ::std::int32_t v0, ::std::int32_t v1, ::std::int32_t v2, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_uniform3iv(::std::int32_t location, ::rust::Slice<::std::int32_t const> value, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_uniform4f(::std::int32_t location, float v0, float v1, float v2, float v3, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_uniform4fv(::std::int32_t location, ::rust::Slice<float const> value, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_uniform4i(::std::int32_t location, ::std::int32_t v0, ::std::int32_t v1, ::std::int32_t v2, ::std::int32_t v3, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_uniform4iv(::std::int32_t location, ::rust::Slice<::std::int32_t const> value, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_uniform_matrix2fv(::std::int32_t location, bool transpose, ::rust::Slice<float const> value, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_uniform_matrix3fv(::std::int32_t location, bool transpose, ::rust::Slice<float const> value, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_uniform_matrix4fv(::std::int32_t location, bool transpose, ::rust::Slice<float const> value, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_use_program(::std::uint32_t program, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_validate_program(::std::uint32_t program, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_vertex_attrib1f(::std::uint32_t index, float v0, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_vertex_attrib1fv(::std::uint32_t index, ::rust::Slice<float const> value, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_vertex_attrib2f(::std::uint32_t index, float v0, float v1, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_vertex_attrib2fv(::std::uint32_t index, ::rust::Slice<float const> value, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_vertex_attrib3f(::std::uint32_t index, float v0, float v1, float v2, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_vertex_attrib3fv(::std::uint32_t index, ::rust::Slice<float const> value, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_vertex_attrib4f(::std::uint32_t index, float v0, float v1, float v2, float v3, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_vertex_attrib4fv(::std::uint32_t index, ::rust::Slice<float const> value, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_vertex_attrib_pointer(::std::uint32_t index, ::std::int32_t size, ::std::uint32_t d_type, bool normalized, ::std::int32_t stride, ::rust::isize offset, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl_viewport(::std::int32_t x, ::std::int32_t y, ::std::int32_t width, ::std::int32_t height, ::org::nativescript::canvas::WebGLState &state) noexcept;
::std::size_t org$nativescript$canvas$cxxbridge1$WebGLSync$operator$sizeof() noexcept;
::std::size_t org$nativescript$canvas$cxxbridge1$WebGLSync$operator$alignof() noexcept;
::std::size_t org$nativescript$canvas$cxxbridge1$WebGLIndexedParameter$operator$sizeof() noexcept;
::std::size_t org$nativescript$canvas$cxxbridge1$WebGLIndexedParameter$operator$alignof() noexcept;

::rust::isize org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_indexed_parameter_get_value(::org::nativescript::canvas::WebGLIndexedParameter const &param) noexcept;

::rust::isize org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_indexed_parameter_get_buffer_value(::org::nativescript::canvas::WebGLIndexedParameter const &param) noexcept;

bool org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_indexed_parameter_get_is_buffer(::org::nativescript::canvas::WebGLIndexedParameter const &param) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_begin_query(::std::uint32_t target, ::std::uint32_t id, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_begin_transform_feedback(::std::uint32_t primitive_mode, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_bind_buffer_base(::std::uint32_t target, ::std::uint32_t index, ::std::uint32_t buffer, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_bind_buffer_range(::std::uint32_t target, ::std::uint32_t index, ::std::uint32_t buffer, ::rust::isize offset, ::rust::isize size, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_bind_sampler(::std::uint32_t unit, ::std::uint32_t sampler, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_bind_transform_feedback(::std::uint32_t target, ::std::uint32_t transform_feedback, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_bind_vertex_array(::std::uint32_t vertex_array, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_blit_framebuffer(::std::int32_t src_x0, ::std::int32_t src_y0, ::std::int32_t src_x1, ::std::int32_t src_y1, ::std::int32_t dst_x0, ::std::int32_t dst_y0, ::std::int32_t dst_x1, ::std::int32_t dst_y1, ::std::uint32_t mask, ::std::uint32_t filter, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_clear_bufferfi(::std::uint32_t buffer, ::std::int32_t drawbuffer, float depth, ::std::int32_t stencil, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_clear_bufferfv(::std::uint32_t buffer, ::std::int32_t drawbuffer, ::rust::Slice<float const> values, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_clear_bufferiv(::std::uint32_t buffer, ::std::int32_t drawbuffer, ::rust::Slice<::std::int32_t const> values, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_clear_bufferuiv(::std::uint32_t buffer, ::std::int32_t drawbuffer, ::rust::Slice<::std::uint32_t const> values, ::org::nativescript::canvas::WebGLState &state) noexcept;

::std::uint32_t org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_client_wait_sync(::org::nativescript::canvas::WebGLSync const &sync, ::std::uint32_t flags, ::rust::isize timeout, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_compressed_tex_sub_image3d_none(::std::uint32_t target, ::std::int32_t level, ::std::int32_t xoffset, ::std::int32_t yoffset, ::std::int32_t zoffset, ::std::int32_t width, ::std::int32_t height, ::std::int32_t depth, ::std::uint32_t format, ::std::int32_t image_size, ::std::int32_t offset, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_compressed_tex_sub_image3d(::std::uint32_t target, ::std::int32_t level, ::std::int32_t xoffset, ::std::int32_t yoffset, ::std::int32_t zoffset, ::std::int32_t width, ::std::int32_t height, ::std::int32_t depth, ::std::uint32_t format, ::rust::Slice<::std::uint8_t const> src, ::std::size_t src_offset, ::std::size_t src_length_override, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_copy_buffer_sub_data(::std::uint32_t read_target, ::std::uint32_t write_target, ::rust::isize read_offset, ::rust::isize write_offset, ::rust::isize size, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_copy_tex_sub_image3d(::std::uint32_t target, ::std::int32_t level, ::std::int32_t xoffset, ::std::int32_t yoffset, ::std::int32_t zoffset, ::std::int32_t x, ::std::int32_t y, ::std::int32_t width, ::std::int32_t height, ::org::nativescript::canvas::WebGLState &state) noexcept;

::std::uint32_t org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_create_query(::org::nativescript::canvas::WebGLState &state) noexcept;

::std::uint32_t org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_create_sampler(::org::nativescript::canvas::WebGLState &state) noexcept;

::std::uint32_t org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_create_transform_feedback(::org::nativescript::canvas::WebGLState &state) noexcept;

::std::uint32_t org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_create_vertex_array(::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_delete_query_with_query(::std::uint32_t id, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_delete_sampler_with_sampler(::std::uint32_t sampler, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_delete_sync_with_sync(::org::nativescript::canvas::WebGLSync const &sync, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_delete_transform_feedback(::std::uint32_t transform_feedback, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_delete_vertex_array_with_vertex_array(::std::uint32_t vertex_array, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_draw_arrays_instanced(::std::uint32_t mode, ::std::int32_t first, ::std::int32_t count, ::std::int32_t instance_count, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_draw_buffers(::rust::Slice<::std::uint32_t const> buffers, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_draw_elements_instanced(::std::uint32_t mode, ::std::int32_t count, ::std::uint32_t type_, ::rust::isize offset, ::std::int32_t instance_count, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_draw_range_elements(::std::uint32_t mode, ::std::uint32_t start, ::std::uint32_t end, ::std::int32_t count, ::std::uint32_t type_, ::rust::isize offset, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_end_query(::std::uint32_t target, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_end_transform_feedback(::org::nativescript::canvas::WebGLState &state) noexcept;

::org::nativescript::canvas::WebGLSync *org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_fence_sync(::std::uint32_t condition, ::std::uint32_t flags, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_framebuffer_texture_layer(::std::uint32_t target, ::std::uint32_t attachment, ::std::uint32_t texture, ::std::int32_t level, ::std::int32_t layer, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_get_active_uniform_block_name(::std::uint32_t program, ::std::uint32_t uniform_block_index, ::org::nativescript::canvas::WebGLState &state, ::rust::String *return$) noexcept;

::org::nativescript::canvas::WebGLResult *org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_get_active_uniform_block_parameter(::std::uint32_t program, ::std::uint32_t uniform_block_index, ::std::uint32_t pname, ::org::nativescript::canvas::WebGLState &state) noexcept;

::org::nativescript::canvas::WebGLResult *org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_get_active_uniforms(::std::uint32_t program, ::rust::Slice<::std::uint32_t const> uniform_indices, ::std::uint32_t pname, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_get_buffer_sub_data(::std::uint32_t target, ::rust::isize src_byte_offset, ::rust::Slice<::std::uint8_t > dst_data, ::std::size_t dst_offset, ::std::size_t length, ::org::nativescript::canvas::WebGLState &state) noexcept;

::std::int32_t org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_get_frag_data_location(::std::uint32_t program, ::rust::Str name, ::org::nativescript::canvas::WebGLState &state) noexcept;

::org::nativescript::canvas::WebGLIndexedParameter *org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_get_indexed_parameter(::std::uint32_t target, ::std::uint32_t index, ::org::nativescript::canvas::WebGLState &state) noexcept;

::org::nativescript::canvas::WebGLResult *org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_get_internalformat_parameter(::std::uint32_t target, ::std::uint32_t internalformat, ::std::uint32_t pname, ::org::nativescript::canvas::WebGLState &state) noexcept;

::org::nativescript::canvas::WebGLResult *org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_get_parameter(::std::uint32_t pname, ::org::nativescript::canvas::WebGLState &state) noexcept;

::org::nativescript::canvas::WebGLResult *org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_get_query_parameter(::std::uint32_t query, ::std::uint32_t pname, ::org::nativescript::canvas::WebGLState &state) noexcept;

::org::nativescript::canvas::WebGLResult *org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_get_query(::std::uint32_t target, ::std::uint32_t pname, ::org::nativescript::canvas::WebGLState &state) noexcept;

::org::nativescript::canvas::WebGLResult *org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_get_sampler_parameter(::std::uint32_t sampler, ::std::uint32_t pname, ::org::nativescript::canvas::WebGLState &state) noexcept;

::org::nativescript::canvas::WebGLResult *org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_get_sync_parameter(::org::nativescript::canvas::WebGLSync const &sync, ::std::uint32_t pname, ::org::nativescript::canvas::WebGLState &state) noexcept;

::org::nativescript::canvas::WebGLActiveInfo *org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_get_transform_feedback_varying(::std::uint32_t program, ::std::uint32_t index, ::org::nativescript::canvas::WebGLState &state) noexcept;

::std::uint32_t org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_get_uniform_block_index(::std::uint32_t program, ::rust::Str uniform_block_name, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_get_uniform_indices(::std::uint32_t program, ::rust::Slice<::rust::Str const> uniform_names, ::org::nativescript::canvas::WebGLState &state, ::rust::Vec<::std::uint32_t> *return$) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_invalidate_framebuffer(::std::uint32_t target, ::rust::Slice<::std::uint32_t const> attachments, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_invalidate_sub_framebuffer(::std::uint32_t target, ::rust::Slice<::std::uint32_t const> attachments, ::std::int32_t x, ::std::int32_t y, ::std::int32_t width, ::std::int32_t height, ::org::nativescript::canvas::WebGLState &state) noexcept;

bool org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_is_query(::std::uint32_t query, ::org::nativescript::canvas::WebGLState &state) noexcept;

bool org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_is_sampler(::std::uint32_t sampler, ::org::nativescript::canvas::WebGLState &state) noexcept;

bool org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_is_sync(::org::nativescript::canvas::WebGLSync const &sync, ::org::nativescript::canvas::WebGLState &state) noexcept;

bool org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_is_transform_feedback(::std::uint32_t transform_feedback, ::org::nativescript::canvas::WebGLState &state) noexcept;

bool org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_is_vertex_array(::std::uint32_t vertex_array, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_pause_transform_feedback(::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_read_buffer(::std::uint32_t src, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_renderbuffer_storage_multisample(::std::uint32_t target, ::std::int32_t samples, ::std::uint32_t internal_format, ::std::int32_t width, ::std::int32_t height, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_resume_transform_feedback(::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_sampler_parameterf(::std::uint32_t sampler, ::std::uint32_t pname, float param, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_sampler_parameteri(::std::uint32_t sampler, ::std::uint32_t pname, ::std::int32_t param, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_tex_image3d_none(::std::uint32_t target, ::std::int32_t level, ::std::int32_t internalformat, ::std::int32_t width, ::std::int32_t height, ::std::int32_t depth, ::std::int32_t border, ::std::uint32_t format, ::std::uint32_t type_, ::std::size_t offset, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_tex_image3d_asset(::std::uint32_t target, ::std::int32_t level, ::std::int32_t internalformat, ::std::int32_t width, ::std::int32_t height, ::std::int32_t depth, ::std::int32_t border, ::std::uint32_t format, ::std::uint32_t type_, ::org::nativescript::canvas::ImageAsset const &asset, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_tex_image3d(::std::uint32_t target, ::std::int32_t level, ::std::int32_t internalformat, ::std::int32_t width, ::std::int32_t height, ::std::int32_t depth, ::std::int32_t border, ::std::uint32_t format, ::std::uint32_t type_, ::rust::Slice<::std::uint8_t const> buf, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_tex_image3d_offset(::std::uint32_t target, ::std::int32_t level, ::std::int32_t internalformat, ::std::int32_t width, ::std::int32_t height, ::std::int32_t depth, ::std::int32_t border, ::std::uint32_t format, ::std::uint32_t type_, ::rust::Slice<::std::uint8_t const> buf, ::std::size_t offset, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_tex_storage2d(::std::uint32_t target, ::std::int32_t levels, ::std::uint32_t internalformat, ::std::int32_t width, ::std::int32_t height, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_tex_storage3d(::std::uint32_t target, ::std::int32_t levels, ::std::uint32_t internalformat, ::std::int32_t width, ::std::int32_t height, ::std::int32_t depth, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_tex_sub_image3d_none(::std::uint32_t target, ::std::int32_t level, ::std::int32_t xoffset, ::std::int32_t yoffset, ::std::int32_t zoffset, ::std::int32_t width, ::std::int32_t height, ::std::int32_t depth, ::std::uint32_t format, ::std::uint32_t type_, ::std::size_t offset, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_tex_sub_image3d(::std::uint32_t target, ::std::int32_t level, ::std::int32_t xoffset, ::std::int32_t yoffset, ::std::int32_t zoffset, ::std::int32_t width, ::std::int32_t height, ::std::int32_t depth, ::std::uint32_t format, ::std::uint32_t type_, ::rust::Slice<::std::uint8_t const> buf, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_tex_sub_image3d_asset(::std::uint32_t target, ::std::int32_t level, ::std::int32_t xoffset, ::std::int32_t yoffset, ::std::int32_t zoffset, ::std::int32_t width, ::std::int32_t height, ::std::int32_t depth, ::std::uint32_t format, ::std::uint32_t type_, ::org::nativescript::canvas::ImageAsset const &asset, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_tex_sub_image3d_offset(::std::uint32_t target, ::std::int32_t level, ::std::int32_t xoffset, ::std::int32_t yoffset, ::std::int32_t zoffset, ::std::int32_t width, ::std::int32_t height, ::std::int32_t depth, ::std::uint32_t format, ::std::uint32_t type_, ::rust::Slice<::std::uint8_t const> buf, ::std::size_t offset, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_transform_feedback_varyings(::std::uint32_t program, ::rust::Slice<::rust::Str const> varyings, ::std::uint32_t buffer_mode, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_uniform1ui(::std::int32_t location, ::std::uint32_t v0, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_uniform1uiv(::std::int32_t location, ::rust::Slice<::std::uint32_t const> data, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_uniform2ui(::std::int32_t location, ::std::uint32_t v0, ::std::uint32_t v1, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_uniform2uiv(::std::int32_t location, ::rust::Slice<::std::uint32_t const> data, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_uniform3ui(::std::int32_t location, ::std::uint32_t v0, ::std::uint32_t v1, ::std::uint32_t v2, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_uniform3uiv(::std::int32_t location, ::rust::Slice<::std::uint32_t const> data, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_uniform4ui(::std::int32_t location, ::std::uint32_t v0, ::std::uint32_t v1, ::std::uint32_t v2, ::std::uint32_t v3, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_uniform4uiv(::std::int32_t location, ::rust::Slice<::std::uint32_t const> data, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_uniform_block_binding(::std::uint32_t program, ::std::uint32_t uniform_block_index, ::std::uint32_t uniform_block_binding, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_uniform_matrix2x3fv(::std::int32_t location, bool transpose, ::rust::Slice<float const> data, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_uniform_matrix2x4fv(::std::int32_t location, bool transpose, ::rust::Slice<float const> data, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_uniform_matrix3x2fv(::std::int32_t location, bool transpose, ::rust::Slice<float const> data, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_uniform_matrix3x4fv(::std::int32_t location, bool transpose, ::rust::Slice<float const> data, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_uniform_matrix4x2fv(::std::int32_t location, bool transpose, ::rust::Slice<float const> data, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_uniform_matrix4x3fv(::std::int32_t location, bool transpose, ::rust::Slice<float const> data, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_vertex_attrib_divisor(::std::uint32_t index, ::std::uint32_t divisor, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_vertex_attrib_i4i(::std::uint32_t index, ::std::int32_t x, ::std::int32_t y, ::std::int32_t z, ::std::int32_t w, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_vertex_attrib_i4iv(::std::uint32_t index, ::rust::Slice<::std::int32_t const> value, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_vertex_attrib_i4ui(::std::uint32_t index, ::std::uint32_t x, ::std::uint32_t y, ::std::uint32_t z, ::std::uint32_t w, ::org::nativescript::canvas::WebGLState &state) noexcept;

void org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_vertex_attrib_i4uiv(::std::uint32_t index, ::rust::Slice<::std::uint32_t const> value, ::org::nativescript::canvas::WebGLState &state) noexcept;
} // extern "C"

::std::size_t FileHelper::layout::size() noexcept {
  return org$nativescript$canvas$cxxbridge1$FileHelper$operator$sizeof();
}

::std::size_t FileHelper::layout::align() noexcept {
  return org$nativescript$canvas$cxxbridge1$FileHelper$operator$alignof();
}

::rust::Box<::org::nativescript::canvas::FileHelper> canvas_native_helper_read_file(::rust::Str path) noexcept {
  return ::rust::Box<::org::nativescript::canvas::FileHelper>::from_raw(org$nativescript$canvas$cxxbridge1$canvas_native_helper_read_file(path));
}

bool canvas_native_helper_read_file_has_error(::org::nativescript::canvas::FileHelper const &file) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_helper_read_file_has_error(file);
}

::rust::Vec<::std::uint8_t> canvas_native_helper_read_file_get_data(::rust::Box<::org::nativescript::canvas::FileHelper> file) noexcept {
  ::rust::MaybeUninit<::rust::Vec<::std::uint8_t>> return$;
  org$nativescript$canvas$cxxbridge1$canvas_native_helper_read_file_get_data(file.into_raw(), &return$.value);
  return ::std::move(return$.value);
}

::rust::String canvas_native_helper_read_file_get_error(::org::nativescript::canvas::FileHelper const &file) noexcept {
  ::rust::MaybeUninit<::rust::String> return$;
  org$nativescript$canvas$cxxbridge1$canvas_native_helper_read_file_get_error(file, &return$.value);
  return ::std::move(return$.value);
}

::std::size_t Raf::layout::size() noexcept {
  return org$nativescript$canvas$cxxbridge1$Raf$operator$sizeof();
}

::std::size_t Raf::layout::align() noexcept {
  return org$nativescript$canvas$cxxbridge1$Raf$operator$alignof();
}

::rust::Box<::org::nativescript::canvas::Raf> canvas_native_raf_create(::rust::isize callback) noexcept {
  return ::rust::Box<::org::nativescript::canvas::Raf>::from_raw(org$nativescript$canvas$cxxbridge1$canvas_native_raf_create(callback));
}

void canvas_native_raf_start(::org::nativescript::canvas::Raf &raf) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_raf_start(raf);
}

void canvas_native_raf_stop(::org::nativescript::canvas::Raf &raf) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_raf_stop(raf);
}

bool canvas_native_raf_get_started(::org::nativescript::canvas::Raf const &raf) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_raf_get_started(raf);
}

::std::size_t CanvasRenderingContext2D::layout::size() noexcept {
  return org$nativescript$canvas$cxxbridge1$CanvasRenderingContext2D$operator$sizeof();
}

::std::size_t CanvasRenderingContext2D::layout::align() noexcept {
  return org$nativescript$canvas$cxxbridge1$CanvasRenderingContext2D$operator$alignof();
}

::std::size_t PaintStyle::layout::size() noexcept {
  return org$nativescript$canvas$cxxbridge1$PaintStyle$operator$sizeof();
}

::std::size_t PaintStyle::layout::align() noexcept {
  return org$nativescript$canvas$cxxbridge1$PaintStyle$operator$alignof();
}

::std::size_t TextMetrics::layout::size() noexcept {
  return org$nativescript$canvas$cxxbridge1$TextMetrics$operator$sizeof();
}

::std::size_t TextMetrics::layout::align() noexcept {
  return org$nativescript$canvas$cxxbridge1$TextMetrics$operator$alignof();
}

::std::size_t Path::layout::size() noexcept {
  return org$nativescript$canvas$cxxbridge1$Path$operator$sizeof();
}

::std::size_t Path::layout::align() noexcept {
  return org$nativescript$canvas$cxxbridge1$Path$operator$alignof();
}

::std::size_t Matrix::layout::size() noexcept {
  return org$nativescript$canvas$cxxbridge1$Matrix$operator$sizeof();
}

::std::size_t Matrix::layout::align() noexcept {
  return org$nativescript$canvas$cxxbridge1$Matrix$operator$alignof();
}

::std::size_t ImageData::layout::size() noexcept {
  return org$nativescript$canvas$cxxbridge1$ImageData$operator$sizeof();
}

::std::size_t ImageData::layout::align() noexcept {
  return org$nativescript$canvas$cxxbridge1$ImageData$operator$alignof();
}

::std::size_t ImageAsset::layout::size() noexcept {
  return org$nativescript$canvas$cxxbridge1$ImageAsset$operator$sizeof();
}

::std::size_t ImageAsset::layout::align() noexcept {
  return org$nativescript$canvas$cxxbridge1$ImageAsset$operator$alignof();
}

::std::size_t TextDecoder::layout::size() noexcept {
  return org$nativescript$canvas$cxxbridge1$TextDecoder$operator$sizeof();
}

::std::size_t TextDecoder::layout::align() noexcept {
  return org$nativescript$canvas$cxxbridge1$TextDecoder$operator$alignof();
}

::std::size_t TextEncoder::layout::size() noexcept {
  return org$nativescript$canvas$cxxbridge1$TextEncoder$operator$sizeof();
}

::std::size_t TextEncoder::layout::align() noexcept {
  return org$nativescript$canvas$cxxbridge1$TextEncoder$operator$alignof();
}

void console_log(::rust::Str text) noexcept {
  org$nativescript$canvas$cxxbridge1$console_log(text);
}

::rust::Vec<::std::uint8_t> str_to_buf(::rust::Str value) noexcept {
  ::rust::MaybeUninit<::rust::Vec<::std::uint8_t>> return$;
  org$nativescript$canvas$cxxbridge1$str_to_buf(value, &return$.value);
  return ::std::move(return$.value);
}

::rust::Box<::org::nativescript::canvas::ImageAsset> canvas_native_image_bitmap_create_from_asset(::org::nativescript::canvas::ImageAsset &asset, bool flip_y, ::org::nativescript::canvas::ImageBitmapPremultiplyAlpha premultiply_alpha, ::org::nativescript::canvas::ImageBitmapColorSpaceConversion color_space_conversion, ::org::nativescript::canvas::ImageBitmapResizeQuality resize_quality, float resize_width, float resize_height) noexcept {
  return ::rust::Box<::org::nativescript::canvas::ImageAsset>::from_raw(org$nativescript$canvas$cxxbridge1$canvas_native_image_bitmap_create_from_asset(asset, flip_y, premultiply_alpha, color_space_conversion, resize_quality, resize_width, resize_height));
}

::rust::Box<::org::nativescript::canvas::ImageAsset> canvas_native_image_bitmap_create_from_asset_src_rect(::org::nativescript::canvas::ImageAsset &asset, float sx, float sy, float s_width, float s_height, bool flip_y, ::org::nativescript::canvas::ImageBitmapPremultiplyAlpha premultiply_alpha, ::org::nativescript::canvas::ImageBitmapColorSpaceConversion color_space_conversion, ::org::nativescript::canvas::ImageBitmapResizeQuality resize_quality, float resize_width, float resize_height) noexcept {
  return ::rust::Box<::org::nativescript::canvas::ImageAsset>::from_raw(org$nativescript$canvas$cxxbridge1$canvas_native_image_bitmap_create_from_asset_src_rect(asset, sx, sy, s_width, s_height, flip_y, premultiply_alpha, color_space_conversion, resize_quality, resize_width, resize_height));
}

::rust::Box<::org::nativescript::canvas::ImageAsset> canvas_native_image_bitmap_create_from_encoded_bytes(::rust::Slice<::std::uint8_t const> bytes, bool flip_y, ::org::nativescript::canvas::ImageBitmapPremultiplyAlpha premultiply_alpha, ::org::nativescript::canvas::ImageBitmapColorSpaceConversion color_space_conversion, ::org::nativescript::canvas::ImageBitmapResizeQuality resize_quality, float resize_width, float resize_height) noexcept {
  return ::rust::Box<::org::nativescript::canvas::ImageAsset>::from_raw(org$nativescript$canvas$cxxbridge1$canvas_native_image_bitmap_create_from_encoded_bytes(bytes, flip_y, premultiply_alpha, color_space_conversion, resize_quality, resize_width, resize_height));
}

bool canvas_native_image_bitmap_create_from_encoded_bytes_with_output(::rust::Slice<::std::uint8_t const> bytes, bool flip_y, ::org::nativescript::canvas::ImageBitmapPremultiplyAlpha premultiply_alpha, ::org::nativescript::canvas::ImageBitmapColorSpaceConversion color_space_conversion, ::org::nativescript::canvas::ImageBitmapResizeQuality resize_quality, float resize_width, float resize_height, ::org::nativescript::canvas::ImageAsset &output) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_image_bitmap_create_from_encoded_bytes_with_output(bytes, flip_y, premultiply_alpha, color_space_conversion, resize_quality, resize_width, resize_height, output);
}

::rust::Box<::org::nativescript::canvas::ImageAsset> canvas_native_image_bitmap_create_from_encoded_bytes_src_rect(::rust::Slice<::std::uint8_t const> bytes, float sx, float sy, float s_width, float s_height, bool flip_y, ::org::nativescript::canvas::ImageBitmapPremultiplyAlpha premultiply_alpha, ::org::nativescript::canvas::ImageBitmapColorSpaceConversion color_space_conversion, ::org::nativescript::canvas::ImageBitmapResizeQuality resize_quality, float resize_width, float resize_height) noexcept {
  return ::rust::Box<::org::nativescript::canvas::ImageAsset>::from_raw(org$nativescript$canvas$cxxbridge1$canvas_native_image_bitmap_create_from_encoded_bytes_src_rect(bytes, sx, sy, s_width, s_height, flip_y, premultiply_alpha, color_space_conversion, resize_quality, resize_width, resize_height));
}

bool canvas_native_image_bitmap_create_from_encoded_bytes_src_rect_with_output(::rust::Slice<::std::uint8_t const> bytes, float sx, float sy, float s_width, float s_height, bool flip_y, ::org::nativescript::canvas::ImageBitmapPremultiplyAlpha premultiply_alpha, ::org::nativescript::canvas::ImageBitmapColorSpaceConversion color_space_conversion, ::org::nativescript::canvas::ImageBitmapResizeQuality resize_quality, float resize_width, float resize_height, ::org::nativescript::canvas::ImageAsset &output) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_image_bitmap_create_from_encoded_bytes_src_rect_with_output(bytes, sx, sy, s_width, s_height, flip_y, premultiply_alpha, color_space_conversion, resize_quality, resize_width, resize_height, output);
}

void canvas_native_path_add_path(::org::nativescript::canvas::Path &path, ::org::nativescript::canvas::Path const &path_to_add) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_path_add_path(path, path_to_add);
}

::rust::Box<::org::nativescript::canvas::Path> canvas_native_path_create() noexcept {
  return ::rust::Box<::org::nativescript::canvas::Path>::from_raw(org$nativescript$canvas$cxxbridge1$canvas_native_path_create());
}

::rust::Box<::org::nativescript::canvas::Path> canvas_native_path_create_with_path(::org::nativescript::canvas::Path const &path) noexcept {
  return ::rust::Box<::org::nativescript::canvas::Path>::from_raw(org$nativescript$canvas$cxxbridge1$canvas_native_path_create_with_path(path));
}

::rust::Box<::org::nativescript::canvas::Path> canvas_native_path_create_with_string(::rust::String string) noexcept {
  return ::rust::Box<::org::nativescript::canvas::Path>::from_raw(org$nativescript$canvas$cxxbridge1$canvas_native_path_create_with_string(&string));
}

::rust::Box<::org::nativescript::canvas::Path> canvas_native_path_create_with_str(::rust::Str string) noexcept {
  return ::rust::Box<::org::nativescript::canvas::Path>::from_raw(org$nativescript$canvas$cxxbridge1$canvas_native_path_create_with_str(string));
}

void canvas_native_path_close_path(::org::nativescript::canvas::Path &path) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_path_close_path(path);
}

void canvas_native_path_move_to(::org::nativescript::canvas::Path &path, float x, float y) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_path_move_to(path, x, y);
}

void canvas_native_path_line_to(::org::nativescript::canvas::Path &path, float x, float y) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_path_line_to(path, x, y);
}

void canvas_native_path_bezier_curve_to(::org::nativescript::canvas::Path &path, float cp1x, float cp1y, float cp2x, float cp2y, float x, float y) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_path_bezier_curve_to(path, cp1x, cp1y, cp2x, cp2y, x, y);
}

void canvas_native_path_quadratic_curve_to(::org::nativescript::canvas::Path &path, float cpx, float cpy, float x, float y) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_path_quadratic_curve_to(path, cpx, cpy, x, y);
}

void canvas_native_path_arc(::org::nativescript::canvas::Path &path, float x, float y, float radius, float start_angle, float end_angle, bool anti_clockwise) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_path_arc(path, x, y, radius, start_angle, end_angle, anti_clockwise);
}

void canvas_native_path_arc_to(::org::nativescript::canvas::Path &path, float x1, float y1, float x2, float y2, float radius) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_path_arc_to(path, x1, y1, x2, y2, radius);
}

void canvas_native_path_ellipse(::org::nativescript::canvas::Path &path, float x, float y, float radius_x, float radius_y, float rotation, float start_angle, float end_angle, bool anticlockwise) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_path_ellipse(path, x, y, radius_x, radius_y, rotation, start_angle, end_angle, anticlockwise);
}

void canvas_native_path_rect(::org::nativescript::canvas::Path &path, float x, float y, float width, float height) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_path_rect(path, x, y, width, height);
}

void canvas_native_path_round_rect(::org::nativescript::canvas::Path &path, float x, float y, float width, float height, ::rust::Slice<float const> radii) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_path_round_rect(path, x, y, width, height, radii);
}

void canvas_native_path_round_rect_tl_tr_br_bl(::org::nativescript::canvas::Path &path, float x, float y, float width, float height, float top_left, float top_right, float bottom_right, float bottom_left) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_path_round_rect_tl_tr_br_bl(path, x, y, width, height, top_left, top_right, bottom_right, bottom_left);
}

::rust::String canvas_native_path_to_string(::org::nativescript::canvas::Path const &path) noexcept {
  ::rust::MaybeUninit<::rust::String> return$;
  org$nativescript$canvas$cxxbridge1$canvas_native_path_to_string(path, &return$.value);
  return ::std::move(return$.value);
}

::rust::Box<::org::nativescript::canvas::Matrix> canvas_native_matrix_create() noexcept {
  return ::rust::Box<::org::nativescript::canvas::Matrix>::from_raw(org$nativescript$canvas$cxxbridge1$canvas_native_matrix_create());
}

void canvas_native_matrix_update(::org::nativescript::canvas::Matrix &matrix, ::rust::Slice<float const> slice) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_matrix_update(matrix, slice);
}

void canvas_native_matrix_update_3d(::org::nativescript::canvas::Matrix &matrix, ::std::array<float, 16> const &slice) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_matrix_update_3d(matrix, slice);
}

float canvas_native_matrix_get_a(::org::nativescript::canvas::Matrix const &matrix) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_matrix_get_a(matrix);
}

void canvas_native_matrix_set_a(::org::nativescript::canvas::Matrix &matrix, float a) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_matrix_set_a(matrix, a);
}

float canvas_native_matrix_get_b(::org::nativescript::canvas::Matrix const &matrix) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_matrix_get_b(matrix);
}

void canvas_native_matrix_set_b(::org::nativescript::canvas::Matrix &matrix, float b) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_matrix_set_b(matrix, b);
}

float canvas_native_matrix_get_c(::org::nativescript::canvas::Matrix const &matrix) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_matrix_get_c(matrix);
}

void canvas_native_matrix_set_c(::org::nativescript::canvas::Matrix &matrix, float c) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_matrix_set_c(matrix, c);
}

float canvas_native_matrix_get_d(::org::nativescript::canvas::Matrix const &matrix) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_matrix_get_d(matrix);
}

void canvas_native_matrix_set_d(::org::nativescript::canvas::Matrix &matrix, float d) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_matrix_set_d(matrix, d);
}

float canvas_native_matrix_get_e(::org::nativescript::canvas::Matrix const &matrix) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_matrix_get_e(matrix);
}

void canvas_native_matrix_set_e(::org::nativescript::canvas::Matrix &matrix, float e) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_matrix_set_e(matrix, e);
}

float canvas_native_matrix_get_f(::org::nativescript::canvas::Matrix const &matrix) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_matrix_get_f(matrix);
}

void canvas_native_matrix_set_f(::org::nativescript::canvas::Matrix &matrix, float f) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_matrix_set_f(matrix, f);
}

float canvas_native_matrix_get_m11(::org::nativescript::canvas::Matrix const &matrix) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_matrix_get_m11(matrix);
}

void canvas_native_matrix_set_m11(::org::nativescript::canvas::Matrix &matrix, float m11) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_matrix_set_m11(matrix, m11);
}

float canvas_native_matrix_get_m12(::org::nativescript::canvas::Matrix const &matrix) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_matrix_get_m12(matrix);
}

void canvas_native_matrix_set_m12(::org::nativescript::canvas::Matrix &matrix, float m12) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_matrix_set_m12(matrix, m12);
}

float canvas_native_matrix_get_m13(::org::nativescript::canvas::Matrix const &matrix) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_matrix_get_m13(matrix);
}

void canvas_native_matrix_set_m13(::org::nativescript::canvas::Matrix &matrix, float m13) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_matrix_set_m13(matrix, m13);
}

float canvas_native_matrix_get_m14(::org::nativescript::canvas::Matrix const &matrix) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_matrix_get_m14(matrix);
}

void canvas_native_matrix_set_m14(::org::nativescript::canvas::Matrix &matrix, float m14) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_matrix_set_m14(matrix, m14);
}

float canvas_native_matrix_get_m21(::org::nativescript::canvas::Matrix const &matrix) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_matrix_get_m21(matrix);
}

void canvas_native_matrix_set_m21(::org::nativescript::canvas::Matrix &matrix, float m21) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_matrix_set_m21(matrix, m21);
}

float canvas_native_matrix_get_m22(::org::nativescript::canvas::Matrix const &matrix) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_matrix_get_m22(matrix);
}

void canvas_native_matrix_set_m22(::org::nativescript::canvas::Matrix &matrix, float m22) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_matrix_set_m22(matrix, m22);
}

float canvas_native_matrix_get_m23(::org::nativescript::canvas::Matrix const &matrix) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_matrix_get_m23(matrix);
}

void canvas_native_matrix_set_m23(::org::nativescript::canvas::Matrix &matrix, float m23) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_matrix_set_m23(matrix, m23);
}

float canvas_native_matrix_get_m24(::org::nativescript::canvas::Matrix const &matrix) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_matrix_get_m24(matrix);
}

void canvas_native_matrix_set_m24(::org::nativescript::canvas::Matrix &matrix, float m24) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_matrix_set_m24(matrix, m24);
}

float canvas_native_matrix_get_m31(::org::nativescript::canvas::Matrix const &matrix) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_matrix_get_m31(matrix);
}

void canvas_native_matrix_set_m31(::org::nativescript::canvas::Matrix &matrix, float m31) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_matrix_set_m31(matrix, m31);
}

float canvas_native_matrix_get_m32(::org::nativescript::canvas::Matrix const &matrix) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_matrix_get_m32(matrix);
}

void canvas_native_matrix_set_m32(::org::nativescript::canvas::Matrix &matrix, float m32) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_matrix_set_m32(matrix, m32);
}

float canvas_native_matrix_get_m33(::org::nativescript::canvas::Matrix const &matrix) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_matrix_get_m33(matrix);
}

void canvas_native_matrix_set_m33(::org::nativescript::canvas::Matrix &matrix, float m33) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_matrix_set_m33(matrix, m33);
}

float canvas_native_matrix_get_m34(::org::nativescript::canvas::Matrix const &matrix) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_matrix_get_m34(matrix);
}

void canvas_native_matrix_set_m34(::org::nativescript::canvas::Matrix &matrix, float m34) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_matrix_set_m34(matrix, m34);
}

float canvas_native_matrix_get_m41(::org::nativescript::canvas::Matrix const &matrix) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_matrix_get_m41(matrix);
}

void canvas_native_matrix_set_m41(::org::nativescript::canvas::Matrix &matrix, float m41) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_matrix_set_m41(matrix, m41);
}

float canvas_native_matrix_get_m42(::org::nativescript::canvas::Matrix const &matrix) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_matrix_get_m42(matrix);
}

void canvas_native_matrix_set_m42(::org::nativescript::canvas::Matrix &matrix, float m42) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_matrix_set_m42(matrix, m42);
}

float canvas_native_matrix_get_m43(::org::nativescript::canvas::Matrix const &matrix) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_matrix_get_m43(matrix);
}

void canvas_native_matrix_set_m43(::org::nativescript::canvas::Matrix &matrix, float m43) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_matrix_set_m43(matrix, m43);
}

float canvas_native_matrix_get_m44(::org::nativescript::canvas::Matrix const &matrix) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_matrix_get_m44(matrix);
}

void canvas_native_matrix_set_m44(::org::nativescript::canvas::Matrix &matrix, float m44) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_matrix_set_m44(matrix, m44);
}

::rust::Box<::org::nativescript::canvas::ImageData> canvas_native_image_data_create(::std::int32_t width, ::std::int32_t height) noexcept {
  return ::rust::Box<::org::nativescript::canvas::ImageData>::from_raw(org$nativescript$canvas$cxxbridge1$canvas_native_image_data_create(width, height));
}

::std::int32_t canvas_native_image_data_get_width(::org::nativescript::canvas::ImageData const &image_data) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_image_data_get_width(image_data);
}

::std::int32_t canvas_native_image_data_get_height(::org::nativescript::canvas::ImageData const &image_data) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_image_data_get_height(image_data);
}

::rust::Slice<::std::uint8_t > canvas_native_image_data_get_data(::org::nativescript::canvas::ImageData &image_data) noexcept {
  return ::rust::impl<::rust::Slice<::std::uint8_t >>::slice(org$nativescript$canvas$cxxbridge1$canvas_native_image_data_get_data(image_data));
}

::rust::Box<::org::nativescript::canvas::ImageData> canvas_native_image_data_get_shared_instance(::org::nativescript::canvas::ImageData &image_data) noexcept {
  return ::rust::Box<::org::nativescript::canvas::ImageData>::from_raw(org$nativescript$canvas$cxxbridge1$canvas_native_image_data_get_shared_instance(image_data));
}

::rust::Box<::org::nativescript::canvas::ImageAsset> canvas_native_image_asset_create() noexcept {
  return ::rust::Box<::org::nativescript::canvas::ImageAsset>::from_raw(org$nativescript$canvas$cxxbridge1$canvas_native_image_asset_create());
}

::rust::Box<::org::nativescript::canvas::ImageAsset> canvas_native_image_asset_shared_clone(::org::nativescript::canvas::ImageAsset const &asset) noexcept {
  return ::rust::Box<::org::nativescript::canvas::ImageAsset>::from_raw(org$nativescript$canvas$cxxbridge1$canvas_native_image_asset_shared_clone(asset));
}

bool canvas_native_image_asset_load_from_fd(::org::nativescript::canvas::ImageAsset &asset, ::std::int32_t fd) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_image_asset_load_from_fd(asset, fd);
}

bool canvas_native_image_asset_load_from_path(::org::nativescript::canvas::ImageAsset &asset, ::rust::Str path) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_image_asset_load_from_path(asset, path);
}

bool canvas_native_image_asset_load_from_url(::org::nativescript::canvas::ImageAsset &asset, ::rust::Str url) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_image_asset_load_from_url(asset, url);
}

bool canvas_native_image_asset_load_from_raw(::org::nativescript::canvas::ImageAsset &asset, ::rust::Slice<::std::uint8_t const> array) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_image_asset_load_from_raw(asset, array);
}

::std::int64_t canvas_native_image_asset_addr(::org::nativescript::canvas::ImageAsset &asset) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_image_asset_addr(asset);
}

::std::uint32_t canvas_native_image_asset_width(::org::nativescript::canvas::ImageAsset &asset) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_image_asset_width(asset);
}

::std::uint32_t canvas_native_image_asset_height(::org::nativescript::canvas::ImageAsset &asset) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_image_asset_height(asset);
}

::rust::String canvas_native_image_asset_get_error(::org::nativescript::canvas::ImageAsset &asset) noexcept {
  ::rust::MaybeUninit<::rust::String> return$;
  org$nativescript$canvas$cxxbridge1$canvas_native_image_asset_get_error(asset, &return$.value);
  return ::std::move(return$.value);
}

bool canvas_native_image_asset_has_error(::org::nativescript::canvas::ImageAsset &asset) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_image_asset_has_error(asset);
}

bool canvas_native_image_asset_scale(::org::nativescript::canvas::ImageAsset &asset, ::std::uint32_t x, ::std::uint32_t y) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_image_asset_scale(asset, x, y);
}

bool canvas_native_image_asset_save_path(::org::nativescript::canvas::ImageAsset &asset, ::rust::Str path, ::std::uint32_t format) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_image_asset_save_path(asset, path, format);
}

float canvas_native_text_metrics_get_width(::org::nativescript::canvas::TextMetrics const &metrics) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_text_metrics_get_width(metrics);
}

float canvas_native_text_metrics_get_actual_bounding_box_left(::org::nativescript::canvas::TextMetrics const &metrics) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_text_metrics_get_actual_bounding_box_left(metrics);
}

float canvas_native_text_metrics_get_actual_bounding_box_right(::org::nativescript::canvas::TextMetrics const &metrics) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_text_metrics_get_actual_bounding_box_right(metrics);
}

float canvas_native_text_metrics_get_actual_bounding_box_ascent(::org::nativescript::canvas::TextMetrics const &metrics) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_text_metrics_get_actual_bounding_box_ascent(metrics);
}

float canvas_native_text_metrics_get_actual_bounding_box_descent(::org::nativescript::canvas::TextMetrics const &metrics) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_text_metrics_get_actual_bounding_box_descent(metrics);
}

float canvas_native_text_metrics_get_font_bounding_box_ascent(::org::nativescript::canvas::TextMetrics const &metrics) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_text_metrics_get_font_bounding_box_ascent(metrics);
}

float canvas_native_text_metrics_get_font_bounding_box_descent(::org::nativescript::canvas::TextMetrics const &metrics) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_text_metrics_get_font_bounding_box_descent(metrics);
}

float canvas_native_text_metrics_get_em_height_ascent(::org::nativescript::canvas::TextMetrics const &metrics) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_text_metrics_get_em_height_ascent(metrics);
}

float canvas_native_text_metrics_get_em_height_descent(::org::nativescript::canvas::TextMetrics const &metrics) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_text_metrics_get_em_height_descent(metrics);
}

float canvas_native_text_metrics_get_hanging_baseline(::org::nativescript::canvas::TextMetrics const &metrics) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_text_metrics_get_hanging_baseline(metrics);
}

float canvas_native_text_metrics_get_alphabetic_baseline(::org::nativescript::canvas::TextMetrics const &metrics) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_text_metrics_get_alphabetic_baseline(metrics);
}

float canvas_native_text_metrics_get_ideographic_baseline(::org::nativescript::canvas::TextMetrics const &metrics) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_text_metrics_get_ideographic_baseline(metrics);
}

::rust::Box<::org::nativescript::canvas::PaintStyle> canvas_native_paint_style_from_bytes(::org::nativescript::canvas::CanvasRenderingContext2D const &context, ::std::int32_t repetition, ::std::int32_t width, ::std::int32_t height, ::rust::Slice<::std::uint8_t const> bytes) noexcept {
  return ::rust::Box<::org::nativescript::canvas::PaintStyle>::from_raw(org$nativescript$canvas$cxxbridge1$canvas_native_paint_style_from_bytes(context, repetition, width, height, bytes));
}

::rust::Box<::org::nativescript::canvas::PaintStyle> canvas_native_pattern_from_ptr(::std::int64_t ptr) noexcept {
  return ::rust::Box<::org::nativescript::canvas::PaintStyle>::from_raw(org$nativescript$canvas$cxxbridge1$canvas_native_pattern_from_ptr(ptr));
}

::rust::Box<::org::nativescript::canvas::PaintStyle> canvas_native_paint_style_empty() noexcept {
  return ::rust::Box<::org::nativescript::canvas::PaintStyle>::from_raw(org$nativescript$canvas$cxxbridge1$canvas_native_paint_style_empty());
}

void canvas_native_gradient_add_color_stop(::org::nativescript::canvas::PaintStyle &style, float stop, ::rust::Str color) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_gradient_add_color_stop(style, stop, color);
}

void canvas_native_pattern_set_transform(::org::nativescript::canvas::PaintStyle &pattern, ::org::nativescript::canvas::Matrix const &matrix) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_pattern_set_transform(pattern, matrix);
}

::rust::Box<::org::nativescript::canvas::TextDecoder> canvas_native_text_decoder_create(::rust::Str decoding) noexcept {
  return ::rust::Box<::org::nativescript::canvas::TextDecoder>::from_raw(org$nativescript$canvas$cxxbridge1$canvas_native_text_decoder_create(decoding));
}

::rust::String canvas_native_text_decoder_get_encoding(::org::nativescript::canvas::TextDecoder &decoder) noexcept {
  ::rust::MaybeUninit<::rust::String> return$;
  org$nativescript$canvas$cxxbridge1$canvas_native_text_decoder_get_encoding(decoder, &return$.value);
  return ::std::move(return$.value);
}

::rust::String canvas_native_text_decoder_decode(::org::nativescript::canvas::TextDecoder &decoder, ::rust::Slice<::std::uint8_t const> data) noexcept {
  ::rust::MaybeUninit<::rust::String> return$;
  org$nativescript$canvas$cxxbridge1$canvas_native_text_decoder_decode(decoder, data, &return$.value);
  return ::std::move(return$.value);
}

::rust::Box<::org::nativescript::canvas::TextEncoder> canvas_native_text_encoder_create(::rust::Str encoding) noexcept {
  return ::rust::Box<::org::nativescript::canvas::TextEncoder>::from_raw(org$nativescript$canvas$cxxbridge1$canvas_native_text_encoder_create(encoding));
}

::rust::String canvas_native_text_encoder_get_encoding(::org::nativescript::canvas::TextEncoder &decoder) noexcept {
  ::rust::MaybeUninit<::rust::String> return$;
  org$nativescript$canvas$cxxbridge1$canvas_native_text_encoder_get_encoding(decoder, &return$.value);
  return ::std::move(return$.value);
}

::rust::Vec<::std::uint8_t> canvas_native_text_encoder_encode(::org::nativescript::canvas::TextEncoder &encoder, ::rust::Str text) noexcept {
  ::rust::MaybeUninit<::rust::Vec<::std::uint8_t>> return$;
  org$nativescript$canvas$cxxbridge1$canvas_native_text_encoder_encode(encoder, text, &return$.value);
  return ::std::move(return$.value);
}

bool canvas_native_context_gl_make_current(::org::nativescript::canvas::CanvasRenderingContext2D const &context) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_context_gl_make_current(context);
}

bool canvas_native_context_gl_swap_buffers(::org::nativescript::canvas::CanvasRenderingContext2D const &context) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_context_gl_swap_buffers(context);
}

::rust::Box<::org::nativescript::canvas::CanvasRenderingContext2D> canvas_native_context_create_with_wrapper(::std::int64_t context, ::std::int64_t gl_context) noexcept {
  return ::rust::Box<::org::nativescript::canvas::CanvasRenderingContext2D>::from_raw(org$nativescript$canvas$cxxbridge1$canvas_native_context_create_with_wrapper(context, gl_context));
}

void canvas_native_context_resize(::org::nativescript::canvas::CanvasRenderingContext2D &context, float width, float height) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_context_resize(context, width, height);
}

::rust::Box<::org::nativescript::canvas::CanvasRenderingContext2D> canvas_native_context_create(float width, float height, float density, bool alpha, ::std::int32_t font_color, float ppi, ::std::uint32_t direction) noexcept {
  return ::rust::Box<::org::nativescript::canvas::CanvasRenderingContext2D>::from_raw(org$nativescript$canvas$cxxbridge1$canvas_native_context_create(width, height, density, alpha, font_color, ppi, direction));
}

::rust::Box<::org::nativescript::canvas::CanvasRenderingContext2D> canvas_native_context_create_gl(float width, float height, float density, ::std::int64_t gl_context, ::std::int32_t samples, bool alpha, ::std::int32_t font_color, float ppi, ::std::uint32_t direction) noexcept {
  return ::rust::Box<::org::nativescript::canvas::CanvasRenderingContext2D>::from_raw(org$nativescript$canvas$cxxbridge1$canvas_native_context_create_gl(width, height, density, gl_context, samples, alpha, font_color, ppi, direction));
}

::rust::Box<::org::nativescript::canvas::CanvasRenderingContext2D> canvas_native_context_create_with_pointer(::std::int64_t pointer) noexcept {
  return ::rust::Box<::org::nativescript::canvas::CanvasRenderingContext2D>::from_raw(org$nativescript$canvas$cxxbridge1$canvas_native_context_create_with_pointer(pointer));
}

::rust::Box<::org::nativescript::canvas::CanvasRenderingContext2D> canvas_native_context_create_gl_no_window(float width, float height, float density, ::std::int32_t font_color, float ppi, ::std::uint32_t direction, bool alpha) noexcept {
  return ::rust::Box<::org::nativescript::canvas::CanvasRenderingContext2D>::from_raw(org$nativescript$canvas$cxxbridge1$canvas_native_context_create_gl_no_window(width, height, density, font_color, ppi, direction, alpha));
}

void canvas_native_context_flush(::org::nativescript::canvas::CanvasRenderingContext2D const &context) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_context_flush(context);
}

void canvas_native_context_render(::org::nativescript::canvas::CanvasRenderingContext2D const &context) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_context_render(context);
}

::rust::String canvas_native_to_data_url(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::rust::Str format, ::std::int32_t quality) noexcept {
  ::rust::MaybeUninit<::rust::String> return$;
  org$nativescript$canvas$cxxbridge1$canvas_native_to_data_url(context, format, quality, &return$.value);
  return ::std::move(return$.value);
}

::rust::String canvas_native_to_data_url_string(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::rust::String format, ::std::int32_t quality) noexcept {
  ::rust::MaybeUninit<::rust::String> return$;
  org$nativescript$canvas$cxxbridge1$canvas_native_to_data_url_string(context, &format, quality, &return$.value);
  return ::std::move(return$.value);
}

::rust::String canvas_native_to_data_url_c_string(::org::nativescript::canvas::CanvasRenderingContext2D &context, char const *format, ::std::int32_t quality) noexcept {
  ::rust::MaybeUninit<::rust::String> return$;
  org$nativescript$canvas$cxxbridge1$canvas_native_to_data_url_c_string(context, format, quality, &return$.value);
  return ::std::move(return$.value);
}

::rust::String canvas_native_context_get_filter(::org::nativescript::canvas::CanvasRenderingContext2D const &context) noexcept {
  ::rust::MaybeUninit<::rust::String> return$;
  org$nativescript$canvas$cxxbridge1$canvas_native_context_get_filter(context, &return$.value);
  return ::std::move(return$.value);
}

void canvas_native_context_set_filter(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::rust::Str font) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_context_set_filter(context, font);
}

::rust::String canvas_native_context_get_font(::org::nativescript::canvas::CanvasRenderingContext2D const &context) noexcept {
  ::rust::MaybeUninit<::rust::String> return$;
  org$nativescript$canvas$cxxbridge1$canvas_native_context_get_font(context, &return$.value);
  return ::std::move(return$.value);
}

void canvas_native_context_set_font(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::rust::Str font) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_context_set_font(context, font);
}

float canvas_native_context_get_global_alpha(::org::nativescript::canvas::CanvasRenderingContext2D const &context) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_context_get_global_alpha(context);
}

void canvas_native_context_set_global_alpha(::org::nativescript::canvas::CanvasRenderingContext2D &context, float alpha) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_context_set_global_alpha(context, alpha);
}

bool canvas_native_context_get_image_smoothing_enabled(::org::nativescript::canvas::CanvasRenderingContext2D const &context) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_context_get_image_smoothing_enabled(context);
}

void canvas_native_context_set_image_smoothing_enabled(::org::nativescript::canvas::CanvasRenderingContext2D &context, bool enabled) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_context_set_image_smoothing_enabled(context, enabled);
}

::rust::Str canvas_native_context_get_image_smoothing_quality(::org::nativescript::canvas::CanvasRenderingContext2D const &context) noexcept {
  return ::rust::impl<::rust::Str>::new_unchecked(org$nativescript$canvas$cxxbridge1$canvas_native_context_get_image_smoothing_quality(context));
}

void canvas_native_context_set_image_smoothing_quality(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::rust::Str quality) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_context_set_image_smoothing_quality(context, quality);
}

::rust::Str canvas_native_context_get_line_join(::org::nativescript::canvas::CanvasRenderingContext2D const &context) noexcept {
  return ::rust::impl<::rust::Str>::new_unchecked(org$nativescript$canvas$cxxbridge1$canvas_native_context_get_line_join(context));
}

void canvas_native_context_set_line_join(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::rust::Str join) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_context_set_line_join(context, join);
}

::rust::Str canvas_native_context_get_line_cap(::org::nativescript::canvas::CanvasRenderingContext2D const &context) noexcept {
  return ::rust::impl<::rust::Str>::new_unchecked(org$nativescript$canvas$cxxbridge1$canvas_native_context_get_line_cap(context));
}

void canvas_native_context_set_line_cap(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::rust::Str cap) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_context_set_line_cap(context, cap);
}

float canvas_native_context_get_miter_limit(::org::nativescript::canvas::CanvasRenderingContext2D const &context) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_context_get_miter_limit(context);
}

void canvas_native_context_set_miter_limit(::org::nativescript::canvas::CanvasRenderingContext2D &context, float limit) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_context_set_miter_limit(context, limit);
}

::rust::String canvas_native_context_get_shadow_color(::org::nativescript::canvas::CanvasRenderingContext2D const &context) noexcept {
  ::rust::MaybeUninit<::rust::String> return$;
  org$nativescript$canvas$cxxbridge1$canvas_native_context_get_shadow_color(context, &return$.value);
  return ::std::move(return$.value);
}

::rust::Vec<::std::uint8_t> canvas_native_context_get_shadow_color_buf(::org::nativescript::canvas::CanvasRenderingContext2D const &context) noexcept {
  ::rust::MaybeUninit<::rust::Vec<::std::uint8_t>> return$;
  org$nativescript$canvas$cxxbridge1$canvas_native_context_get_shadow_color_buf(context, &return$.value);
  return ::std::move(return$.value);
}

void canvas_native_context_get_shadow_color_rgba(::org::nativescript::canvas::CanvasRenderingContext2D const &context, ::std::uint8_t &r, ::std::uint8_t &g, ::std::uint8_t &b, ::std::uint8_t &a) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_context_get_shadow_color_rgba(context, r, g, b, a);
}

void canvas_native_context_set_shadow_color(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::rust::Str color) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_context_set_shadow_color(context, color);
}

void canvas_native_context_set_shadow_color_rgba(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::std::uint8_t r, ::std::uint8_t g, ::std::uint8_t b, ::std::uint8_t a) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_context_set_shadow_color_rgba(context, r, g, b, a);
}

float canvas_native_context_get_shadow_blur(::org::nativescript::canvas::CanvasRenderingContext2D const &context) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_context_get_shadow_blur(context);
}

void canvas_native_context_set_shadow_blur(::org::nativescript::canvas::CanvasRenderingContext2D &context, float blur) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_context_set_shadow_blur(context, blur);
}

float canvas_native_context_get_shadow_offset_x(::org::nativescript::canvas::CanvasRenderingContext2D const &context) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_context_get_shadow_offset_x(context);
}

void canvas_native_context_set_shadow_offset_x(::org::nativescript::canvas::CanvasRenderingContext2D &context, float x) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_context_set_shadow_offset_x(context, x);
}

float canvas_native_context_get_shadow_offset_y(::org::nativescript::canvas::CanvasRenderingContext2D const &context) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_context_get_shadow_offset_y(context);
}

void canvas_native_context_set_shadow_offset_y(::org::nativescript::canvas::CanvasRenderingContext2D &context, float y) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_context_set_shadow_offset_y(context, y);
}

::rust::Str canvas_native_context_get_text_align(::org::nativescript::canvas::CanvasRenderingContext2D const &context) noexcept {
  return ::rust::impl<::rust::Str>::new_unchecked(org$nativescript$canvas$cxxbridge1$canvas_native_context_get_text_align(context));
}

void canvas_native_context_set_text_align(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::rust::Str alignment) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_context_set_text_align(context, alignment);
}

::rust::Str canvas_native_context_get_global_composition(::org::nativescript::canvas::CanvasRenderingContext2D const &context) noexcept {
  return ::rust::impl<::rust::Str>::new_unchecked(org$nativescript$canvas$cxxbridge1$canvas_native_context_get_global_composition(context));
}

void canvas_native_context_set_global_composition(::org::nativescript::canvas::CanvasRenderingContext2D const &context, ::rust::Str composition) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_context_set_global_composition(context, composition);
}

void canvas_native_paint_style_set_fill_color_with_string(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::rust::Str color) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_paint_style_set_fill_color_with_string(context, color);
}

bool canvas_native_parse_css_color_rgba(::rust::Str value, ::std::uint8_t &r, ::std::uint8_t &g, ::std::uint8_t &b, ::std::uint8_t &a) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_parse_css_color_rgba(value, r, g, b, a);
}

void canvas_native_paint_style_set_stroke_color_with_rgba(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::std::uint8_t r, ::std::uint8_t g, ::std::uint8_t b, ::std::uint8_t a) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_paint_style_set_stroke_color_with_rgba(context, r, g, b, a);
}

void canvas_native_paint_style_set_fill_color_with_rgba(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::std::uint8_t r, ::std::uint8_t g, ::std::uint8_t b, ::std::uint8_t a) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_paint_style_set_fill_color_with_rgba(context, r, g, b, a);
}

void canvas_native_paint_style_set_stroke_color_with_string(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::rust::Str color) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_paint_style_set_stroke_color_with_string(context, color);
}

::rust::String canvas_native_paint_style_get_color_string(::org::nativescript::canvas::PaintStyle &color) noexcept {
  ::rust::MaybeUninit<::rust::String> return$;
  org$nativescript$canvas$cxxbridge1$canvas_native_paint_style_get_color_string(color, &return$.value);
  return ::std::move(return$.value);
}

::rust::String canvas_native_paint_style_get_current_stroke_color_string(::org::nativescript::canvas::CanvasRenderingContext2D &context) noexcept {
  ::rust::MaybeUninit<::rust::String> return$;
  org$nativescript$canvas$cxxbridge1$canvas_native_paint_style_get_current_stroke_color_string(context, &return$.value);
  return ::std::move(return$.value);
}

::rust::Vec<::std::uint8_t> canvas_native_paint_style_get_current_stroke_color_buf(::org::nativescript::canvas::CanvasRenderingContext2D &context) noexcept {
  ::rust::MaybeUninit<::rust::Vec<::std::uint8_t>> return$;
  org$nativescript$canvas$cxxbridge1$canvas_native_paint_style_get_current_stroke_color_buf(context, &return$.value);
  return ::std::move(return$.value);
}

void canvas_native_paint_style_get_current_stroke_color_r_g_b_a(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::std::uint8_t &r, ::std::uint8_t &g, ::std::uint8_t &b, ::std::uint8_t &a) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_paint_style_get_current_stroke_color_r_g_b_a(context, r, g, b, a);
}

::rust::String canvas_native_paint_style_get_current_fill_color_string(::org::nativescript::canvas::CanvasRenderingContext2D &context) noexcept {
  ::rust::MaybeUninit<::rust::String> return$;
  org$nativescript$canvas$cxxbridge1$canvas_native_paint_style_get_current_fill_color_string(context, &return$.value);
  return ::std::move(return$.value);
}

::rust::Vec<::std::uint8_t> canvas_native_paint_style_get_current_fill_color_buf(::org::nativescript::canvas::CanvasRenderingContext2D &context) noexcept {
  ::rust::MaybeUninit<::rust::Vec<::std::uint8_t>> return$;
  org$nativescript$canvas$cxxbridge1$canvas_native_paint_style_get_current_fill_color_buf(context, &return$.value);
  return ::std::move(return$.value);
}

void canvas_native_paint_style_get_current_fill_color_r_g_b_a(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::std::uint8_t &r, ::std::uint8_t &g, ::std::uint8_t &b, ::std::uint8_t &a) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_paint_style_get_current_fill_color_r_g_b_a(context, r, g, b, a);
}

::org::nativescript::canvas::PaintStyleType canvas_native_context_get_style_type(::org::nativescript::canvas::PaintStyle const &style) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_context_get_style_type(style);
}

::org::nativescript::canvas::PaintStyleType canvas_native_context_get_current_fill_style_type(::org::nativescript::canvas::CanvasRenderingContext2D &context) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_context_get_current_fill_style_type(context);
}

::org::nativescript::canvas::PaintStyleType canvas_native_context_get_current_stroke_style_type(::org::nativescript::canvas::CanvasRenderingContext2D &context) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_context_get_current_stroke_style_type(context);
}

::rust::Box<::org::nativescript::canvas::PaintStyle> canvas_native_context_get_fill_style(::org::nativescript::canvas::CanvasRenderingContext2D &context) noexcept {
  return ::rust::Box<::org::nativescript::canvas::PaintStyle>::from_raw(org$nativescript$canvas$cxxbridge1$canvas_native_context_get_fill_style(context));
}

void canvas_native_context_set_fill_style(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::org::nativescript::canvas::PaintStyle const &style) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_context_set_fill_style(context, style);
}

::rust::Box<::org::nativescript::canvas::PaintStyle> canvas_native_context_get_stroke_style(::org::nativescript::canvas::CanvasRenderingContext2D const &context) noexcept {
  return ::rust::Box<::org::nativescript::canvas::PaintStyle>::from_raw(org$nativescript$canvas$cxxbridge1$canvas_native_context_get_stroke_style(context));
}

void canvas_native_context_set_stroke_style(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::org::nativescript::canvas::PaintStyle const &style) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_context_set_stroke_style(context, style);
}

float canvas_native_context_get_line_width(::org::nativescript::canvas::CanvasRenderingContext2D const &context) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_context_get_line_width(context);
}

void canvas_native_context_set_line_width(::org::nativescript::canvas::CanvasRenderingContext2D &context, float width) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_context_set_line_width(context, width);
}

float canvas_native_context_get_line_dash_offset(::org::nativescript::canvas::CanvasRenderingContext2D const &context) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_context_get_line_dash_offset(context);
}

void canvas_native_context_set_line_dash_offset(::org::nativescript::canvas::CanvasRenderingContext2D &context, float offset) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_context_set_line_dash_offset(context, offset);
}

::rust::Vec<float> canvas_native_context_get_line_dash(::org::nativescript::canvas::CanvasRenderingContext2D const &context) noexcept {
  ::rust::MaybeUninit<::rust::Vec<float>> return$;
  org$nativescript$canvas$cxxbridge1$canvas_native_context_get_line_dash(context, &return$.value);
  return ::std::move(return$.value);
}

void canvas_native_context_set_line_dash(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::rust::Slice<float const> dash) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_context_set_line_dash(context, dash);
}

void canvas_native_context_arc(::org::nativescript::canvas::CanvasRenderingContext2D &context, float x, float y, float radius, float start_angle, float end_angle, bool anticlockwise) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_context_arc(context, x, y, radius, start_angle, end_angle, anticlockwise);
}

void canvas_native_context_arc_to(::org::nativescript::canvas::CanvasRenderingContext2D &context, float x1, float y1, float x2, float y2, float radius) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_context_arc_to(context, x1, y1, x2, y2, radius);
}

void canvas_native_context_begin_path(::org::nativescript::canvas::CanvasRenderingContext2D &context) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_context_begin_path(context);
}

void canvas_native_context_bezier_curve_to(::org::nativescript::canvas::CanvasRenderingContext2D &context, float cp1x, float cp1y, float cp2x, float cp2y, float x, float y) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_context_bezier_curve_to(context, cp1x, cp1y, cp2x, cp2y, x, y);
}

void canvas_native_context_clear_rect(::org::nativescript::canvas::CanvasRenderingContext2D &context, float x, float y, float width, float height) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_context_clear_rect(context, x, y, width, height);
}

void canvas_native_context_clip(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::org::nativescript::canvas::Path &path, ::rust::Str rule) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_context_clip(context, path, rule);
}

void canvas_native_context_clip_rule(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::rust::Str rule) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_context_clip_rule(context, rule);
}

void canvas_native_context_close_path(::org::nativescript::canvas::CanvasRenderingContext2D &context) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_context_close_path(context);
}

::rust::Box<::org::nativescript::canvas::ImageData> canvas_native_context_create_image_data(::std::int32_t width, ::std::int32_t height) noexcept {
  return ::rust::Box<::org::nativescript::canvas::ImageData>::from_raw(org$nativescript$canvas$cxxbridge1$canvas_native_context_create_image_data(width, height));
}

::rust::Box<::org::nativescript::canvas::ImageData> canvas_native_context_create_image_data_with_data(::std::int32_t width, ::std::int32_t height, ::rust::Slice<::std::uint8_t const> data) noexcept {
  return ::rust::Box<::org::nativescript::canvas::ImageData>::from_raw(org$nativescript$canvas$cxxbridge1$canvas_native_context_create_image_data_with_data(width, height, data));
}

::rust::Box<::org::nativescript::canvas::PaintStyle> canvas_native_context_create_linear_gradient(::org::nativescript::canvas::CanvasRenderingContext2D &context, float x0, float y0, float x1, float y1) noexcept {
  return ::rust::Box<::org::nativescript::canvas::PaintStyle>::from_raw(org$nativescript$canvas$cxxbridge1$canvas_native_context_create_linear_gradient(context, x0, y0, x1, y1));
}

::rust::Box<::org::nativescript::canvas::PaintStyle> canvas_native_context_create_pattern(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::rust::Slice<::std::uint8_t const> data, ::std::int32_t width, ::std::int32_t height, ::rust::Str repetition) noexcept {
  return ::rust::Box<::org::nativescript::canvas::PaintStyle>::from_raw(org$nativescript$canvas$cxxbridge1$canvas_native_context_create_pattern(context, data, width, height, repetition));
}

::rust::Box<::org::nativescript::canvas::PaintStyle> canvas_native_context_create_pattern_asset(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::org::nativescript::canvas::ImageAsset &asset, ::rust::Str repetition) noexcept {
  return ::rust::Box<::org::nativescript::canvas::PaintStyle>::from_raw(org$nativescript$canvas$cxxbridge1$canvas_native_context_create_pattern_asset(context, asset, repetition));
}

::rust::Box<::org::nativescript::canvas::PaintStyle> canvas_native_context_create_pattern_canvas2d(::org::nativescript::canvas::CanvasRenderingContext2D &source, ::org::nativescript::canvas::CanvasRenderingContext2D &context, ::rust::Str repetition) noexcept {
  return ::rust::Box<::org::nativescript::canvas::PaintStyle>::from_raw(org$nativescript$canvas$cxxbridge1$canvas_native_context_create_pattern_canvas2d(source, context, repetition));
}

::rust::Box<::org::nativescript::canvas::PaintStyle> canvas_native_context_create_pattern_encoded(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::rust::Slice<::std::uint8_t const> data, ::rust::Str repetition) noexcept {
  return ::rust::Box<::org::nativescript::canvas::PaintStyle>::from_raw(org$nativescript$canvas$cxxbridge1$canvas_native_context_create_pattern_encoded(context, data, repetition));
}

::rust::Box<::org::nativescript::canvas::PaintStyle> canvas_native_context_create_radial_gradient(::org::nativescript::canvas::CanvasRenderingContext2D &context, float x0, float y0, float r0, float x1, float y1, float r1) noexcept {
  return ::rust::Box<::org::nativescript::canvas::PaintStyle>::from_raw(org$nativescript$canvas$cxxbridge1$canvas_native_context_create_radial_gradient(context, x0, y0, r0, x1, y1, r1));
}

void canvas_native_context_draw_paint(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::rust::Str color) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_context_draw_paint(context, color);
}

void canvas_native_context_draw_point(::org::nativescript::canvas::CanvasRenderingContext2D &context, float x, float y) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_context_draw_point(context, x, y);
}

void canvas_native_context_draw_points(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::std::int32_t mode, ::rust::Slice<float const> points) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_context_draw_points(context, mode, points);
}

void canvas_native_context_draw_image_dx_dy(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::rust::Slice<::std::uint8_t const> data, float width, float height, float dx, float dy) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_context_draw_image_dx_dy(context, data, width, height, dx, dy);
}

void canvas_native_context_draw_image_dx_dy_dw_dh(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::rust::Slice<::std::uint8_t const> data, float width, float height, float dx, float dy, float d_width, float d_height) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_context_draw_image_dx_dy_dw_dh(context, data, width, height, dx, dy, d_width, d_height);
}

void canvas_native_context_draw_image(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::rust::Slice<::std::uint8_t const> data, float width, float height, float sx, float sy, float s_width, float s_height, float dx, float dy, float d_width, float d_height) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_context_draw_image(context, data, width, height, sx, sy, s_width, s_height, dx, dy, d_width, d_height);
}

void canvas_native_context_draw_image_encoded_dx_dy(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::rust::Slice<::std::uint8_t const> data, float dx, float dy) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_context_draw_image_encoded_dx_dy(context, data, dx, dy);
}

void canvas_native_context_draw_image_encoded_dx_dy_dw_dh(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::rust::Slice<::std::uint8_t const> data, float dx, float dy, float d_width, float d_height) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_context_draw_image_encoded_dx_dy_dw_dh(context, data, dx, dy, d_width, d_height);
}

void canvas_native_context_draw_image_encoded(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::rust::Slice<::std::uint8_t const> data, float sx, float sy, float s_width, float s_height, float dx, float dy, float d_width, float d_height) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_context_draw_image_encoded(context, data, sx, sy, s_width, s_height, dx, dy, d_width, d_height);
}

void canvas_native_context_draw_image_dx_dy_context(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::org::nativescript::canvas::CanvasRenderingContext2D &source, float dx, float dy) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_context_draw_image_dx_dy_context(context, source, dx, dy);
}

void canvas_native_context_draw_image_dx_dy_dw_dh_context(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::org::nativescript::canvas::CanvasRenderingContext2D &source, float dx, float dy, float d_width, float d_height) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_context_draw_image_dx_dy_dw_dh_context(context, source, dx, dy, d_width, d_height);
}

void canvas_native_context_draw_image_context(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::org::nativescript::canvas::CanvasRenderingContext2D &source, float sx, float sy, float s_width, float s_height, float dx, float dy, float d_width, float d_height) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_context_draw_image_context(context, source, sx, sy, s_width, s_height, dx, dy, d_width, d_height);
}

void canvas_native_context_draw_image_dx_dy_asset(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::org::nativescript::canvas::ImageAsset &asset, float dx, float dy) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_context_draw_image_dx_dy_asset(context, asset, dx, dy);
}

void canvas_native_context_draw_image_dx_dy_dw_dh_asset(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::org::nativescript::canvas::ImageAsset &asset, float dx, float dy, float d_width, float d_height) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_context_draw_image_dx_dy_dw_dh_asset(context, asset, dx, dy, d_width, d_height);
}

void canvas_native_context_draw_image_asset(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::org::nativescript::canvas::ImageAsset &asset, float sx, float sy, float s_width, float s_height, float dx, float dy, float d_width, float d_height) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_context_draw_image_asset(context, asset, sx, sy, s_width, s_height, dx, dy, d_width, d_height);
}

void canvas_native_context_ellipse(::org::nativescript::canvas::CanvasRenderingContext2D &context, float x, float y, float radius_x, float radius_y, float rotation, float start_angle, float end_angle, bool anticlockwise) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_context_ellipse(context, x, y, radius_x, radius_y, rotation, start_angle, end_angle, anticlockwise);
}

void canvas_native_context_fill(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::rust::Str rule) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_context_fill(context, rule);
}

void canvas_native_context_fill_with_path(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::org::nativescript::canvas::Path &path, ::rust::Str rule) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_context_fill_with_path(context, path, rule);
}

void canvas_native_context_fill_rect(::org::nativescript::canvas::CanvasRenderingContext2D &context, float x, float y, float width, float height) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_context_fill_rect(context, x, y, width, height);
}

void canvas_native_context_fill_text(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::rust::Str text, float x, float y, float width) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_context_fill_text(context, text, x, y, width);
}

::rust::Box<::org::nativescript::canvas::ImageData> canvas_native_context_get_image_data(::org::nativescript::canvas::CanvasRenderingContext2D &context, float sx, float sy, float sw, float sh) noexcept {
  return ::rust::Box<::org::nativescript::canvas::ImageData>::from_raw(org$nativescript$canvas$cxxbridge1$canvas_native_context_get_image_data(context, sx, sy, sw, sh));
}

::rust::Box<::org::nativescript::canvas::Matrix> canvas_native_context_get_transform(::org::nativescript::canvas::CanvasRenderingContext2D &context) noexcept {
  return ::rust::Box<::org::nativescript::canvas::Matrix>::from_raw(org$nativescript$canvas$cxxbridge1$canvas_native_context_get_transform(context));
}

bool canvas_native_context_is_point_in_path(::org::nativescript::canvas::CanvasRenderingContext2D &context, float x, float y, ::rust::Str rule) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_context_is_point_in_path(context, x, y, rule);
}

bool canvas_native_context_is_point_in_path_with_path(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::org::nativescript::canvas::Path &path, float x, float y, ::rust::Str rule) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_context_is_point_in_path_with_path(context, path, x, y, rule);
}

bool canvas_native_context_is_point_in_stroke(::org::nativescript::canvas::CanvasRenderingContext2D &context, float x, float y) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_context_is_point_in_stroke(context, x, y);
}

bool canvas_native_context_is_point_in_stroke_with_path(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::org::nativescript::canvas::Path &path, float x, float y) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_context_is_point_in_stroke_with_path(context, path, x, y);
}

void canvas_native_context_line_to(::org::nativescript::canvas::CanvasRenderingContext2D &context, float x, float y) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_context_line_to(context, x, y);
}

::rust::Box<::org::nativescript::canvas::TextMetrics> canvas_native_context_measure_text(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::rust::Str text) noexcept {
  return ::rust::Box<::org::nativescript::canvas::TextMetrics>::from_raw(org$nativescript$canvas$cxxbridge1$canvas_native_context_measure_text(context, text));
}

void canvas_native_context_move_to(::org::nativescript::canvas::CanvasRenderingContext2D &context, float x, float y) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_context_move_to(context, x, y);
}

void canvas_native_context_put_image_data(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::org::nativescript::canvas::ImageData &image_data, float dx, float dy, float dirty_x, float dirty_y, float dirty_width, float dirty_height) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_context_put_image_data(context, image_data, dx, dy, dirty_x, dirty_y, dirty_width, dirty_height);
}

void canvas_native_context_quadratic_curve_to(::org::nativescript::canvas::CanvasRenderingContext2D &context, float cpx, float cpy, float x, float y) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_context_quadratic_curve_to(context, cpx, cpy, x, y);
}

void canvas_native_context_rect(::org::nativescript::canvas::CanvasRenderingContext2D &context, float x, float y, float width, float height) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_context_rect(context, x, y, width, height);
}

void canvas_native_context_round_rect(::org::nativescript::canvas::CanvasRenderingContext2D &context, float x, float y, float width, float height, ::rust::Slice<float const> radii) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_context_round_rect(context, x, y, width, height, radii);
}

void canvas_native_context_round_rect_tl_tr_br_bl(::org::nativescript::canvas::CanvasRenderingContext2D &context, float x, float y, float width, float height, float top_left, float top_right, float bottom_right, float bottom_left) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_context_round_rect_tl_tr_br_bl(context, x, y, width, height, top_left, top_right, bottom_right, bottom_left);
}

void canvas_native_context_reset_transform(::org::nativescript::canvas::CanvasRenderingContext2D &context) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_context_reset_transform(context);
}

void canvas_native_context_restore(::org::nativescript::canvas::CanvasRenderingContext2D &context) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_context_restore(context);
}

void canvas_native_context_rotate(::org::nativescript::canvas::CanvasRenderingContext2D &context, float angle) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_context_rotate(context, angle);
}

void canvas_native_context_save(::org::nativescript::canvas::CanvasRenderingContext2D &context) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_context_save(context);
}

void canvas_native_context_scale(::org::nativescript::canvas::CanvasRenderingContext2D &context, float x, float y) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_context_scale(context, x, y);
}

void canvas_native_context_set_transform(::org::nativescript::canvas::CanvasRenderingContext2D &context, float a, float b, float c, float d, float e, float f) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_context_set_transform(context, a, b, c, d, e, f);
}

void canvas_native_context_set_transform_matrix(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::org::nativescript::canvas::Matrix &matrix) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_context_set_transform_matrix(context, matrix);
}

void canvas_native_context_stroke(::org::nativescript::canvas::CanvasRenderingContext2D &context) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_context_stroke(context);
}

void canvas_native_context_stroke_with_path(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::org::nativescript::canvas::Path &path) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_context_stroke_with_path(context, path);
}

void canvas_native_context_stroke_rect(::org::nativescript::canvas::CanvasRenderingContext2D &context, float x, float y, float width, float height) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_context_stroke_rect(context, x, y, width, height);
}

void canvas_native_context_stroke_text(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::rust::Str text, float x, float y, float width) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_context_stroke_text(context, text, x, y, width);
}

void canvas_native_context_transform(::org::nativescript::canvas::CanvasRenderingContext2D &context, float a, float b, float c, float d, float e, float f) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_context_transform(context, a, b, c, d, e, f);
}

void canvas_native_context_translate(::org::nativescript::canvas::CanvasRenderingContext2D &context, float x, float y) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_context_translate(context, x, y);
}

::std::size_t WebGLState::layout::size() noexcept {
  return org$nativescript$canvas$cxxbridge1$WebGLState$operator$sizeof();
}

::std::size_t WebGLState::layout::align() noexcept {
  return org$nativescript$canvas$cxxbridge1$WebGLState$operator$alignof();
}

::std::size_t WebGLActiveInfo::layout::size() noexcept {
  return org$nativescript$canvas$cxxbridge1$WebGLActiveInfo$operator$sizeof();
}

::std::size_t WebGLActiveInfo::layout::align() noexcept {
  return org$nativescript$canvas$cxxbridge1$WebGLActiveInfo$operator$alignof();
}

::std::size_t WebGLResult::layout::size() noexcept {
  return org$nativescript$canvas$cxxbridge1$WebGLResult$operator$sizeof();
}

::std::size_t WebGLResult::layout::align() noexcept {
  return org$nativescript$canvas$cxxbridge1$WebGLResult$operator$alignof();
}

::std::size_t ContextAttributes::layout::size() noexcept {
  return org$nativescript$canvas$cxxbridge1$ContextAttributes$operator$sizeof();
}

::std::size_t ContextAttributes::layout::align() noexcept {
  return org$nativescript$canvas$cxxbridge1$ContextAttributes$operator$alignof();
}

::std::size_t WebGLExtension::layout::size() noexcept {
  return org$nativescript$canvas$cxxbridge1$WebGLExtension$operator$sizeof();
}

::std::size_t WebGLExtension::layout::align() noexcept {
  return org$nativescript$canvas$cxxbridge1$WebGLExtension$operator$alignof();
}

::std::size_t WebGLFramebufferAttachmentParameter::layout::size() noexcept {
  return org$nativescript$canvas$cxxbridge1$WebGLFramebufferAttachmentParameter$operator$sizeof();
}

::std::size_t WebGLFramebufferAttachmentParameter::layout::align() noexcept {
  return org$nativescript$canvas$cxxbridge1$WebGLFramebufferAttachmentParameter$operator$alignof();
}

::std::size_t WebGLShaderPrecisionFormat::layout::size() noexcept {
  return org$nativescript$canvas$cxxbridge1$WebGLShaderPrecisionFormat$operator$sizeof();
}

::std::size_t WebGLShaderPrecisionFormat::layout::align() noexcept {
  return org$nativescript$canvas$cxxbridge1$WebGLShaderPrecisionFormat$operator$alignof();
}

::std::size_t EXT_blend_minmax::layout::size() noexcept {
  return org$nativescript$canvas$cxxbridge1$EXT_blend_minmax$operator$sizeof();
}

::std::size_t EXT_blend_minmax::layout::align() noexcept {
  return org$nativescript$canvas$cxxbridge1$EXT_blend_minmax$operator$alignof();
}

::std::size_t EXT_color_buffer_half_float::layout::size() noexcept {
  return org$nativescript$canvas$cxxbridge1$EXT_color_buffer_half_float$operator$sizeof();
}

::std::size_t EXT_color_buffer_half_float::layout::align() noexcept {
  return org$nativescript$canvas$cxxbridge1$EXT_color_buffer_half_float$operator$alignof();
}

::std::size_t EXT_disjoint_timer_query::layout::size() noexcept {
  return org$nativescript$canvas$cxxbridge1$EXT_disjoint_timer_query$operator$sizeof();
}

::std::size_t EXT_disjoint_timer_query::layout::align() noexcept {
  return org$nativescript$canvas$cxxbridge1$EXT_disjoint_timer_query$operator$alignof();
}

::std::size_t EXT_sRGB::layout::size() noexcept {
  return org$nativescript$canvas$cxxbridge1$EXT_sRGB$operator$sizeof();
}

::std::size_t EXT_sRGB::layout::align() noexcept {
  return org$nativescript$canvas$cxxbridge1$EXT_sRGB$operator$alignof();
}

::std::size_t EXT_shader_texture_lod::layout::size() noexcept {
  return org$nativescript$canvas$cxxbridge1$EXT_shader_texture_lod$operator$sizeof();
}

::std::size_t EXT_shader_texture_lod::layout::align() noexcept {
  return org$nativescript$canvas$cxxbridge1$EXT_shader_texture_lod$operator$alignof();
}

::std::size_t EXT_texture_filter_anisotropic::layout::size() noexcept {
  return org$nativescript$canvas$cxxbridge1$EXT_texture_filter_anisotropic$operator$sizeof();
}

::std::size_t EXT_texture_filter_anisotropic::layout::align() noexcept {
  return org$nativescript$canvas$cxxbridge1$EXT_texture_filter_anisotropic$operator$alignof();
}

::std::size_t OES_element_index_uint::layout::size() noexcept {
  return org$nativescript$canvas$cxxbridge1$OES_element_index_uint$operator$sizeof();
}

::std::size_t OES_element_index_uint::layout::align() noexcept {
  return org$nativescript$canvas$cxxbridge1$OES_element_index_uint$operator$alignof();
}

::std::size_t OES_standard_derivatives::layout::size() noexcept {
  return org$nativescript$canvas$cxxbridge1$OES_standard_derivatives$operator$sizeof();
}

::std::size_t OES_standard_derivatives::layout::align() noexcept {
  return org$nativescript$canvas$cxxbridge1$OES_standard_derivatives$operator$alignof();
}

::std::size_t OES_texture_float::layout::size() noexcept {
  return org$nativescript$canvas$cxxbridge1$OES_texture_float$operator$sizeof();
}

::std::size_t OES_texture_float::layout::align() noexcept {
  return org$nativescript$canvas$cxxbridge1$OES_texture_float$operator$alignof();
}

::std::size_t OES_texture_float_linear::layout::size() noexcept {
  return org$nativescript$canvas$cxxbridge1$OES_texture_float_linear$operator$sizeof();
}

::std::size_t OES_texture_float_linear::layout::align() noexcept {
  return org$nativescript$canvas$cxxbridge1$OES_texture_float_linear$operator$alignof();
}

::std::size_t OES_texture_half_float::layout::size() noexcept {
  return org$nativescript$canvas$cxxbridge1$OES_texture_half_float$operator$sizeof();
}

::std::size_t OES_texture_half_float::layout::align() noexcept {
  return org$nativescript$canvas$cxxbridge1$OES_texture_half_float$operator$alignof();
}

::std::size_t OES_texture_half_float_linear::layout::size() noexcept {
  return org$nativescript$canvas$cxxbridge1$OES_texture_half_float_linear$operator$sizeof();
}

::std::size_t OES_texture_half_float_linear::layout::align() noexcept {
  return org$nativescript$canvas$cxxbridge1$OES_texture_half_float_linear$operator$alignof();
}

::std::size_t OES_vertex_array_object::layout::size() noexcept {
  return org$nativescript$canvas$cxxbridge1$OES_vertex_array_object$operator$sizeof();
}

::std::size_t OES_vertex_array_object::layout::align() noexcept {
  return org$nativescript$canvas$cxxbridge1$OES_vertex_array_object$operator$alignof();
}

::std::size_t WEBGL_color_buffer_float::layout::size() noexcept {
  return org$nativescript$canvas$cxxbridge1$WEBGL_color_buffer_float$operator$sizeof();
}

::std::size_t WEBGL_color_buffer_float::layout::align() noexcept {
  return org$nativescript$canvas$cxxbridge1$WEBGL_color_buffer_float$operator$alignof();
}

::std::size_t WEBGL_compressed_texture_atc::layout::size() noexcept {
  return org$nativescript$canvas$cxxbridge1$WEBGL_compressed_texture_atc$operator$sizeof();
}

::std::size_t WEBGL_compressed_texture_atc::layout::align() noexcept {
  return org$nativescript$canvas$cxxbridge1$WEBGL_compressed_texture_atc$operator$alignof();
}

::std::size_t WEBGL_compressed_texture_etc1::layout::size() noexcept {
  return org$nativescript$canvas$cxxbridge1$WEBGL_compressed_texture_etc1$operator$sizeof();
}

::std::size_t WEBGL_compressed_texture_etc1::layout::align() noexcept {
  return org$nativescript$canvas$cxxbridge1$WEBGL_compressed_texture_etc1$operator$alignof();
}

::std::size_t WEBGL_compressed_texture_s3tc::layout::size() noexcept {
  return org$nativescript$canvas$cxxbridge1$WEBGL_compressed_texture_s3tc$operator$sizeof();
}

::std::size_t WEBGL_compressed_texture_s3tc::layout::align() noexcept {
  return org$nativescript$canvas$cxxbridge1$WEBGL_compressed_texture_s3tc$operator$alignof();
}

::std::size_t WEBGL_compressed_texture_s3tc_srgb::layout::size() noexcept {
  return org$nativescript$canvas$cxxbridge1$WEBGL_compressed_texture_s3tc_srgb$operator$sizeof();
}

::std::size_t WEBGL_compressed_texture_s3tc_srgb::layout::align() noexcept {
  return org$nativescript$canvas$cxxbridge1$WEBGL_compressed_texture_s3tc_srgb$operator$alignof();
}

::std::size_t WEBGL_compressed_texture_etc::layout::size() noexcept {
  return org$nativescript$canvas$cxxbridge1$WEBGL_compressed_texture_etc$operator$sizeof();
}

::std::size_t WEBGL_compressed_texture_etc::layout::align() noexcept {
  return org$nativescript$canvas$cxxbridge1$WEBGL_compressed_texture_etc$operator$alignof();
}

::std::size_t WEBGL_compressed_texture_pvrtc::layout::size() noexcept {
  return org$nativescript$canvas$cxxbridge1$WEBGL_compressed_texture_pvrtc$operator$sizeof();
}

::std::size_t WEBGL_compressed_texture_pvrtc::layout::align() noexcept {
  return org$nativescript$canvas$cxxbridge1$WEBGL_compressed_texture_pvrtc$operator$alignof();
}

::std::size_t WEBGL_lose_context::layout::size() noexcept {
  return org$nativescript$canvas$cxxbridge1$WEBGL_lose_context$operator$sizeof();
}

::std::size_t WEBGL_lose_context::layout::align() noexcept {
  return org$nativescript$canvas$cxxbridge1$WEBGL_lose_context$operator$alignof();
}

::std::size_t ANGLE_instanced_arrays::layout::size() noexcept {
  return org$nativescript$canvas$cxxbridge1$ANGLE_instanced_arrays$operator$sizeof();
}

::std::size_t ANGLE_instanced_arrays::layout::align() noexcept {
  return org$nativescript$canvas$cxxbridge1$ANGLE_instanced_arrays$operator$alignof();
}

::std::size_t WEBGL_depth_texture::layout::size() noexcept {
  return org$nativescript$canvas$cxxbridge1$WEBGL_depth_texture$operator$sizeof();
}

::std::size_t WEBGL_depth_texture::layout::align() noexcept {
  return org$nativescript$canvas$cxxbridge1$WEBGL_depth_texture$operator$alignof();
}

::std::size_t WEBGL_draw_buffers::layout::size() noexcept {
  return org$nativescript$canvas$cxxbridge1$WEBGL_draw_buffers$operator$sizeof();
}

::std::size_t WEBGL_draw_buffers::layout::align() noexcept {
  return org$nativescript$canvas$cxxbridge1$WEBGL_draw_buffers$operator$alignof();
}

::rust::Box<::org::nativescript::canvas::PaintStyle> canvas_native_context_create_pattern_webgl(::org::nativescript::canvas::WebGLState &source, ::org::nativescript::canvas::CanvasRenderingContext2D &context, ::rust::Str repetition) noexcept {
  return ::rust::Box<::org::nativescript::canvas::PaintStyle>::from_raw(org$nativescript$canvas$cxxbridge1$canvas_native_context_create_pattern_webgl(source, context, repetition));
}

bool canvas_native_webgl_make_current(::org::nativescript::canvas::WebGLState &state) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_webgl_make_current(state);
}

bool canvas_native_webgl_swap_buffers(::org::nativescript::canvas::WebGLState &state) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_webgl_swap_buffers(state);
}

::rust::String canvas_native_webgl_to_data_url(::org::nativescript::canvas::WebGLState &state, ::rust::Str format, ::std::int32_t quality) noexcept {
  ::rust::MaybeUninit<::rust::String> return$;
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_to_data_url(state, format, quality, &return$.value);
  return ::std::move(return$.value);
}

void canvas_native_webgl_resized(::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_resized(state);
}

::rust::Str canvas_native_webgl_active_info_get_name(::org::nativescript::canvas::WebGLActiveInfo const &info) noexcept {
  return ::rust::impl<::rust::Str>::new_unchecked(org$nativescript$canvas$cxxbridge1$canvas_native_webgl_active_info_get_name(info));
}

::std::int32_t canvas_native_webgl_active_info_get_size(::org::nativescript::canvas::WebGLActiveInfo const &info) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_webgl_active_info_get_size(info);
}

::std::uint32_t canvas_native_webgl_active_info_get_type(::org::nativescript::canvas::WebGLActiveInfo const &info) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_webgl_active_info_get_type(info);
}

bool canvas_native_webgl_active_info_get_is_empty(::org::nativescript::canvas::WebGLActiveInfo const &info) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_webgl_active_info_get_is_empty(info);
}

::std::int32_t canvas_native_webgl_shader_precision_format_get_range_min(::org::nativescript::canvas::WebGLShaderPrecisionFormat const &shader) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_webgl_shader_precision_format_get_range_min(shader);
}

::std::int32_t canvas_native_webgl_shader_precision_format_get_range_max(::org::nativescript::canvas::WebGLShaderPrecisionFormat const &shader) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_webgl_shader_precision_format_get_range_max(shader);
}

::std::int32_t canvas_native_webgl_shader_precision_format_get_precision(::org::nativescript::canvas::WebGLShaderPrecisionFormat const &shader) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_webgl_shader_precision_format_get_precision(shader);
}

bool canvas_native_webgl_context_attribute_get_get_alpha(::org::nativescript::canvas::ContextAttributes const &attr) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_webgl_context_attribute_get_get_alpha(attr);
}

bool canvas_native_webgl_context_attribute_get_get_antialias(::org::nativescript::canvas::ContextAttributes const &attr) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_webgl_context_attribute_get_get_antialias(attr);
}

bool canvas_native_webgl_context_attribute_get_get_depth(::org::nativescript::canvas::ContextAttributes const &attr) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_webgl_context_attribute_get_get_depth(attr);
}

bool canvas_native_webgl_context_attribute_get_get_fail_if_major_performance_caveat(::org::nativescript::canvas::ContextAttributes const &attr) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_webgl_context_attribute_get_get_fail_if_major_performance_caveat(attr);
}

::rust::String canvas_native_webgl_context_attribute_get_get_power_preference(::org::nativescript::canvas::ContextAttributes const &attr) noexcept {
  ::rust::MaybeUninit<::rust::String> return$;
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_context_attribute_get_get_power_preference(attr, &return$.value);
  return ::std::move(return$.value);
}

bool canvas_native_webgl_context_attribute_get_get_premultiplied_alpha(::org::nativescript::canvas::ContextAttributes const &attr) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_webgl_context_attribute_get_get_premultiplied_alpha(attr);
}

bool canvas_native_webgl_context_attribute_get_get_preserve_drawing_buffer(::org::nativescript::canvas::ContextAttributes const &attr) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_webgl_context_attribute_get_get_preserve_drawing_buffer(attr);
}

bool canvas_native_webgl_context_attribute_get_get_stencil(::org::nativescript::canvas::ContextAttributes const &attr) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_webgl_context_attribute_get_get_stencil(attr);
}

bool canvas_native_webgl_context_attribute_get_get_desynchronized(::org::nativescript::canvas::ContextAttributes const &attr) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_webgl_context_attribute_get_get_desynchronized(attr);
}

bool canvas_native_webgl_context_attribute_get_get_xr_compatible(::org::nativescript::canvas::ContextAttributes const &attr) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_webgl_context_attribute_get_get_xr_compatible(attr);
}

bool canvas_native_webgl_context_extension_is_none(::org::nativescript::canvas::WebGLExtension const &extension) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_webgl_context_extension_is_none(extension);
}

::org::nativescript::canvas::WebGLExtensionType canvas_native_webgl_context_extension_get_type(::org::nativescript::canvas::WebGLExtension const &extension) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_webgl_context_extension_get_type(extension);
}

::rust::Box<::org::nativescript::canvas::EXT_disjoint_timer_query> canvas_native_webgl_context_extension_to_ext_disjoint_timer_query(::rust::Box<::org::nativescript::canvas::WebGLExtension> extension) noexcept {
  return ::rust::Box<::org::nativescript::canvas::EXT_disjoint_timer_query>::from_raw(org$nativescript$canvas$cxxbridge1$canvas_native_webgl_context_extension_to_ext_disjoint_timer_query(extension.into_raw()));
}

::rust::Box<::org::nativescript::canvas::ANGLE_instanced_arrays> canvas_native_webgl_context_extension_to_angle_instanced_arrays(::rust::Box<::org::nativescript::canvas::WebGLExtension> extension) noexcept {
  return ::rust::Box<::org::nativescript::canvas::ANGLE_instanced_arrays>::from_raw(org$nativescript$canvas$cxxbridge1$canvas_native_webgl_context_extension_to_angle_instanced_arrays(extension.into_raw()));
}

::rust::Box<::org::nativescript::canvas::WEBGL_lose_context> canvas_native_webgl_context_extension_to_lose_context(::rust::Box<::org::nativescript::canvas::WebGLExtension> extension) noexcept {
  return ::rust::Box<::org::nativescript::canvas::WEBGL_lose_context>::from_raw(org$nativescript$canvas$cxxbridge1$canvas_native_webgl_context_extension_to_lose_context(extension.into_raw()));
}

::rust::Box<::org::nativescript::canvas::WEBGL_draw_buffers> canvas_native_webgl_context_extension_to_draw_buffers(::rust::Box<::org::nativescript::canvas::WebGLExtension> extension) noexcept {
  return ::rust::Box<::org::nativescript::canvas::WEBGL_draw_buffers>::from_raw(org$nativescript$canvas$cxxbridge1$canvas_native_webgl_context_extension_to_draw_buffers(extension.into_raw()));
}

::rust::Box<::org::nativescript::canvas::OES_vertex_array_object> canvas_native_webgl_context_extension_to_oes_vertex_array_object(::rust::Box<::org::nativescript::canvas::WebGLExtension> extension) noexcept {
  return ::rust::Box<::org::nativescript::canvas::OES_vertex_array_object>::from_raw(org$nativescript$canvas$cxxbridge1$canvas_native_webgl_context_extension_to_oes_vertex_array_object(extension.into_raw()));
}

void canvas_native_webgl_lose_context_lose_context(::org::nativescript::canvas::WEBGL_lose_context const &context) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_lose_context_lose_context(context);
}

void canvas_native_webgl_lose_context_restore_context(::org::nativescript::canvas::WEBGL_lose_context const &context) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_lose_context_restore_context(context);
}

void canvas_native_webgl_draw_buffers_draw_buffers_webgl(::rust::Slice<::std::uint32_t const> buffers, ::org::nativescript::canvas::WEBGL_draw_buffers const &context) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_draw_buffers_draw_buffers_webgl(buffers, context);
}

::std::uint32_t canvas_native_webgl_oes_vertex_array_object_create_vertex_array_oes(::org::nativescript::canvas::OES_vertex_array_object const &object) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_webgl_oes_vertex_array_object_create_vertex_array_oes(object);
}

void canvas_native_webgl_oes_vertex_array_object_delete_vertex_array_oes(::std::uint32_t array_object, ::org::nativescript::canvas::OES_vertex_array_object const &object) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_oes_vertex_array_object_delete_vertex_array_oes(array_object, object);
}

bool canvas_native_webgl_oes_vertex_array_object_is_vertex_array_oes(::std::uint32_t array_object, ::org::nativescript::canvas::OES_vertex_array_object const &object) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_webgl_oes_vertex_array_object_is_vertex_array_oes(array_object, object);
}

void canvas_native_webgl_oes_vertex_array_object_bind_vertex_array_oes(::std::uint32_t array_object, ::org::nativescript::canvas::OES_vertex_array_object const &object) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_oes_vertex_array_object_bind_vertex_array_oes(array_object, object);
}

bool canvas_native_webgl_framebuffer_attachment_parameter_get_is_texture(::org::nativescript::canvas::WebGLFramebufferAttachmentParameter const &param) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_webgl_framebuffer_attachment_parameter_get_is_texture(param);
}

bool canvas_native_webgl_framebuffer_attachment_parameter_get_is_renderbuffer(::org::nativescript::canvas::WebGLFramebufferAttachmentParameter const &param) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_webgl_framebuffer_attachment_parameter_get_is_renderbuffer(param);
}

::std::int32_t canvas_native_webgl_framebuffer_attachment_parameter_get_value(::org::nativescript::canvas::WebGLFramebufferAttachmentParameter const &param) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_webgl_framebuffer_attachment_parameter_get_value(param);
}

::org::nativescript::canvas::WebGLResultType canvas_native_webgl_result_get_type(::org::nativescript::canvas::WebGLResult const &result) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_webgl_result_get_type(result);
}

bool canvas_native_webgl_result_get_bool(::org::nativescript::canvas::WebGLResult const &result) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_webgl_result_get_bool(result);
}

::rust::Vec<::std::int32_t> canvas_native_webgl_result_get_i32_array(::org::nativescript::canvas::WebGLResult const &result) noexcept {
  ::rust::MaybeUninit<::rust::Vec<::std::int32_t>> return$;
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_result_get_i32_array(result, &return$.value);
  return ::std::move(return$.value);
}

::rust::Vec<::std::uint32_t> canvas_native_webgl_result_get_u32_array(::org::nativescript::canvas::WebGLResult const &result) noexcept {
  ::rust::MaybeUninit<::rust::Vec<::std::uint32_t>> return$;
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_result_get_u32_array(result, &return$.value);
  return ::std::move(return$.value);
}

::rust::Vec<float> canvas_native_webgl_result_get_f32_array(::org::nativescript::canvas::WebGLResult const &result) noexcept {
  ::rust::MaybeUninit<::rust::Vec<float>> return$;
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_result_get_f32_array(result, &return$.value);
  return ::std::move(return$.value);
}

::rust::Vec<::std::uint8_t> canvas_native_webgl_result_get_bool_array(::org::nativescript::canvas::WebGLResult const &result) noexcept {
  ::rust::MaybeUninit<::rust::Vec<::std::uint8_t>> return$;
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_result_get_bool_array(result, &return$.value);
  return ::std::move(return$.value);
}

::std::uint32_t canvas_native_webgl_result_get_u32(::org::nativescript::canvas::WebGLResult const &result) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_webgl_result_get_u32(result);
}

::std::int32_t canvas_native_webgl_result_get_i32(::org::nativescript::canvas::WebGLResult const &result) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_webgl_result_get_i32(result);
}

float canvas_native_webgl_result_get_f32(::org::nativescript::canvas::WebGLResult const &result) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_webgl_result_get_f32(result);
}

::rust::String canvas_native_webgl_result_get_string(::org::nativescript::canvas::WebGLResult const &result) noexcept {
  ::rust::MaybeUninit<::rust::String> return$;
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_result_get_string(result, &return$.value);
  return ::std::move(return$.value);
}

bool canvas_native_webgl_result_get_is_none(::org::nativescript::canvas::WebGLResult const &result) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_webgl_result_get_is_none(result);
}

::std::int32_t canvas_native_webgl_state_get_unpack_colorspace_conversion_webgl(::org::nativescript::canvas::WebGLState &state) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_webgl_state_get_unpack_colorspace_conversion_webgl(state);
}

bool canvas_native_webgl_state_get_flip_y(::org::nativescript::canvas::WebGLState &state) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_webgl_state_get_flip_y(state);
}

bool canvas_native_webgl_state_get_premultiplied_alpha(::org::nativescript::canvas::WebGLState &state) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_webgl_state_get_premultiplied_alpha(state);
}

::std::int32_t canvas_native_webgl_state_get_drawing_buffer_width(::org::nativescript::canvas::WebGLState &state) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_webgl_state_get_drawing_buffer_width(state);
}

::std::int32_t canvas_native_webgl_state_get_drawing_buffer_height(::org::nativescript::canvas::WebGLState &state) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_webgl_state_get_drawing_buffer_height(state);
}

::std::uint32_t canvas_native_webgl_ext_disjoint_timer_query_create_query_ext(::org::nativescript::canvas::EXT_disjoint_timer_query const &query) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_webgl_ext_disjoint_timer_query_create_query_ext(query);
}

void canvas_native_webgl_ext_disjoint_timer_query_delete_query_ext(::std::uint32_t value, ::org::nativescript::canvas::EXT_disjoint_timer_query const &query) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_ext_disjoint_timer_query_delete_query_ext(value, query);
}

bool canvas_native_webgl_ext_disjoint_timer_query_is_query_ext(::std::uint32_t value, ::org::nativescript::canvas::EXT_disjoint_timer_query const &query) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_webgl_ext_disjoint_timer_query_is_query_ext(value, query);
}

void canvas_native_webgl_ext_disjoint_timer_query_begin_query_ext(::std::uint32_t target, ::std::uint32_t value, ::org::nativescript::canvas::EXT_disjoint_timer_query const &query) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_ext_disjoint_timer_query_begin_query_ext(target, value, query);
}

void canvas_native_webgl_ext_disjoint_timer_query_end_query_ext(::std::uint32_t target, ::org::nativescript::canvas::EXT_disjoint_timer_query const &query) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_ext_disjoint_timer_query_end_query_ext(target, query);
}

void canvas_native_webgl_ext_disjoint_timer_query_query_counter_ext(::std::uint32_t value, ::std::uint32_t target, ::org::nativescript::canvas::EXT_disjoint_timer_query const &query) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_ext_disjoint_timer_query_query_counter_ext(value, target, query);
}

::std::int32_t canvas_native_webgl_ext_disjoint_timer_query_get_query_ext(::std::uint32_t target, ::std::uint32_t pname, ::org::nativescript::canvas::EXT_disjoint_timer_query const &query) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_webgl_ext_disjoint_timer_query_get_query_ext(target, pname, query);
}

::rust::Box<::org::nativescript::canvas::WebGLResult> canvas_native_webgl_ext_disjoint_timer_query_get_query_object_ext(::std::uint32_t target, ::std::uint32_t pname, ::org::nativescript::canvas::EXT_disjoint_timer_query const &query) noexcept {
  return ::rust::Box<::org::nativescript::canvas::WebGLResult>::from_raw(org$nativescript$canvas$cxxbridge1$canvas_native_webgl_ext_disjoint_timer_query_get_query_object_ext(target, pname, query));
}

void canvas_native_webgl_angle_instanced_arrays_draw_arrays_instanced_angle(::std::uint32_t mode, ::std::int32_t first, ::std::int32_t count, ::std::int32_t primcount, ::org::nativescript::canvas::ANGLE_instanced_arrays const &arrays) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_angle_instanced_arrays_draw_arrays_instanced_angle(mode, first, count, primcount, arrays);
}

void canvas_native_webgl_angle_instanced_arrays_draw_elements_instanced_angle(::std::uint32_t mode, ::std::int32_t count, ::std::uint32_t type_, ::std::int32_t offset, ::std::int32_t primcount, ::org::nativescript::canvas::ANGLE_instanced_arrays const &arrays) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_angle_instanced_arrays_draw_elements_instanced_angle(mode, count, type_, offset, primcount, arrays);
}

void canvas_native_webgl_angle_instanced_arrays_vertex_attrib_divisor_angle(::std::uint32_t index, ::std::uint32_t divisor, ::org::nativescript::canvas::ANGLE_instanced_arrays const &arrays) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_angle_instanced_arrays_vertex_attrib_divisor_angle(index, divisor, arrays);
}

::rust::Box<::org::nativescript::canvas::WebGLState> canvas_native_webgl_create(::std::int64_t gl_context, ::rust::Str version, bool alpha, bool antialias, bool depth, bool fail_if_major_performance_caveat, ::rust::Str power_preference, bool premultiplied_alpha, bool preserve_drawing_buffer, bool stencil, bool desynchronized, bool xr_compatible) noexcept {
  return ::rust::Box<::org::nativescript::canvas::WebGLState>::from_raw(org$nativescript$canvas$cxxbridge1$canvas_native_webgl_create(gl_context, version, alpha, antialias, depth, fail_if_major_performance_caveat, power_preference, premultiplied_alpha, preserve_drawing_buffer, stencil, desynchronized, xr_compatible));
}

::rust::Box<::org::nativescript::canvas::WebGLState> canvas_native_webgl_create_no_window(::std::int32_t width, ::std::int32_t height, ::rust::Str version, bool alpha, bool antialias, bool depth, bool fail_if_major_performance_caveat, ::rust::Str power_preference, bool premultiplied_alpha, bool preserve_drawing_buffer, bool stencil, bool desynchronized, bool xr_compatible, bool is_canvas) noexcept {
  return ::rust::Box<::org::nativescript::canvas::WebGLState>::from_raw(org$nativescript$canvas$cxxbridge1$canvas_native_webgl_create_no_window(width, height, version, alpha, antialias, depth, fail_if_major_performance_caveat, power_preference, premultiplied_alpha, preserve_drawing_buffer, stencil, desynchronized, xr_compatible, is_canvas));
}

void canvas_native_webgl_active_texture(::std::uint32_t texture, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_active_texture(texture, state);
}

void canvas_native_webgl_attach_shader(::std::uint32_t program, ::std::uint32_t shader, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_attach_shader(program, shader, state);
}

void canvas_native_webgl_bind_attrib_location(::std::uint32_t program, ::std::uint32_t index, ::rust::Str name, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_bind_attrib_location(program, index, name, state);
}

void canvas_native_webgl_bind_buffer(::std::uint32_t target, ::std::uint32_t buffer, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_bind_buffer(target, buffer, state);
}

void canvas_native_webgl_bind_frame_buffer(::std::uint32_t target, ::std::uint32_t framebuffer, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_bind_frame_buffer(target, framebuffer, state);
}

void canvas_native_webgl_bind_render_buffer(::std::uint32_t target, ::std::uint32_t renderbuffer, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_bind_render_buffer(target, renderbuffer, state);
}

void canvas_native_webgl_bind_texture(::std::uint32_t target, ::std::uint32_t texture, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_bind_texture(target, texture, state);
}

void canvas_native_webgl_blend_color(float red, float green, float blue, float alpha, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_blend_color(red, green, blue, alpha, state);
}

void canvas_native_webgl_blend_equation_separate(::std::uint32_t mode_rgb, ::std::uint32_t mode_alpha, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_blend_equation_separate(mode_rgb, mode_alpha, state);
}

void canvas_native_webgl_blend_equation(::std::uint32_t mode, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_blend_equation(mode, state);
}

void canvas_native_webgl_blend_func_separate(::std::uint32_t src_rgb, ::std::uint32_t dst_rgb, ::std::uint32_t src_alpha, ::std::uint32_t dst_alpha, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_blend_func_separate(src_rgb, dst_rgb, src_alpha, dst_alpha, state);
}

void canvas_native_webgl_blend_func(::std::uint32_t sfactor, ::std::uint32_t dfactor, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_blend_func(sfactor, dfactor, state);
}

void canvas_native_webgl_buffer_data(::std::uint32_t target, ::rust::Slice<::std::uint8_t const> src_data, ::std::uint32_t usage, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_buffer_data(target, src_data, usage, state);
}

void canvas_native_webgl_buffer_data_u16(::std::uint32_t target, ::rust::Slice<::std::uint16_t const> src_data, ::std::uint32_t usage, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_buffer_data_u16(target, src_data, usage, state);
}

void canvas_native_webgl_buffer_data_f32(::std::uint32_t target, ::rust::Slice<float const> src_data, ::std::uint32_t usage, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_buffer_data_f32(target, src_data, usage, state);
}

void canvas_native_webgl_buffer_data_none(::std::uint32_t target, ::rust::isize size, ::std::uint32_t usage, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_buffer_data_none(target, size, usage, state);
}

void canvas_native_webgl_buffer_sub_data(::std::uint32_t target, ::rust::isize offset, ::rust::Slice<::std::uint8_t const> src_data, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_buffer_sub_data(target, offset, src_data, state);
}

void canvas_native_webgl_buffer_sub_data_none(::std::uint32_t target, ::rust::isize offset, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_buffer_sub_data_none(target, offset, state);
}

::std::uint32_t canvas_native_webgl_check_frame_buffer_status(::std::uint32_t target, ::org::nativescript::canvas::WebGLState &state) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_webgl_check_frame_buffer_status(target, state);
}

void canvas_native_webgl_clear(::std::uint32_t mask, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_clear(mask, state);
}

void canvas_native_webgl_clear_color(float red, float green, float blue, float alpha, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_clear_color(red, green, blue, alpha, state);
}

void canvas_native_webgl_clear_depth(float depth, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_clear_depth(depth, state);
}

void canvas_native_webgl_clear_stencil(::std::int32_t stencil, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_clear_stencil(stencil, state);
}

void canvas_native_webgl_color_mask(bool red, bool green, bool blue, bool alpha, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_color_mask(red, green, blue, alpha, state);
}

void canvas_native_webgl_commit(::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_commit(state);
}

void canvas_native_webgl_compile_shader(::std::uint32_t shader, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_compile_shader(shader, state);
}

void canvas_native_webgl_compressed_tex_image2d(::std::uint32_t target, ::std::int32_t level, ::std::uint32_t internalformat, ::std::int32_t width, ::std::int32_t height, ::std::int32_t border, ::rust::Slice<::std::uint8_t const> pixels, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_compressed_tex_image2d(target, level, internalformat, width, height, border, pixels, state);
}

void canvas_native_webgl_compressed_tex_image2d_none(::std::uint32_t target, ::std::int32_t level, ::std::uint32_t internalformat, ::std::int32_t width, ::std::int32_t height, ::std::int32_t border, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_compressed_tex_image2d_none(target, level, internalformat, width, height, border, state);
}

void canvas_native_webgl_compressed_tex_sub_image2d(::std::uint32_t target, ::std::int32_t level, ::std::int32_t xoffset, ::std::int32_t yoffset, ::std::int32_t width, ::std::int32_t height, ::std::uint32_t format, ::rust::Slice<::std::uint8_t const> pixels, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_compressed_tex_sub_image2d(target, level, xoffset, yoffset, width, height, format, pixels, state);
}

void canvas_native_webgl_copy_tex_image2d(::std::uint32_t target, ::std::int32_t level, ::std::uint32_t internalformat, ::std::int32_t x, ::std::int32_t y, ::std::int32_t width, ::std::int32_t height, ::std::int32_t border, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_copy_tex_image2d(target, level, internalformat, x, y, width, height, border, state);
}

void canvas_native_webgl_copy_tex_sub_image2d(::std::uint32_t target, ::std::int32_t level, ::std::int32_t xoffset, ::std::int32_t yoffset, ::std::int32_t x, ::std::int32_t y, ::std::int32_t width, ::std::int32_t height, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_copy_tex_sub_image2d(target, level, xoffset, yoffset, x, y, width, height, state);
}

::std::uint32_t canvas_native_webgl_create_buffer(::org::nativescript::canvas::WebGLState &state) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_webgl_create_buffer(state);
}

::std::uint32_t canvas_native_webgl_create_framebuffer(::org::nativescript::canvas::WebGLState &state) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_webgl_create_framebuffer(state);
}

::std::uint32_t canvas_native_webgl_create_program(::org::nativescript::canvas::WebGLState &state) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_webgl_create_program(state);
}

::std::uint32_t canvas_native_webgl_create_renderbuffer(::org::nativescript::canvas::WebGLState &state) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_webgl_create_renderbuffer(state);
}

::std::uint32_t canvas_native_webgl_create_shader(::std::uint32_t shader_type, ::org::nativescript::canvas::WebGLState &state) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_webgl_create_shader(shader_type, state);
}

::std::uint32_t canvas_native_webgl_create_texture(::org::nativescript::canvas::WebGLState &state) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_webgl_create_texture(state);
}

void canvas_native_webgl_cull_face(::std::uint32_t mode, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_cull_face(mode, state);
}

void canvas_native_webgl_delete_buffer(::std::uint32_t buffer, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_delete_buffer(buffer, state);
}

void canvas_native_webgl_delete_framebuffer(::std::uint32_t frame_buffer, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_delete_framebuffer(frame_buffer, state);
}

void canvas_native_webgl_delete_program(::std::uint32_t program, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_delete_program(program, state);
}

void canvas_native_webgl_delete_renderbuffer(::std::uint32_t render_buffer, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_delete_renderbuffer(render_buffer, state);
}

void canvas_native_webgl_delete_shader(::std::uint32_t shader, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_delete_shader(shader, state);
}

void canvas_native_webgl_delete_texture(::std::uint32_t texture, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_delete_texture(texture, state);
}

void canvas_native_webgl_depth_func(::std::uint32_t func, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_depth_func(func, state);
}

void canvas_native_webgl_depth_mask(bool flag, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_depth_mask(flag, state);
}

void canvas_native_webgl_depth_range(float z_near, float z_far, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_depth_range(z_near, z_far, state);
}

void canvas_native_webgl_detach_shader(::std::uint32_t program, ::std::uint32_t shader, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_detach_shader(program, shader, state);
}

void canvas_native_webgl_disable(::std::uint32_t cap, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_disable(cap, state);
}

void canvas_native_webgl_disable_vertex_attrib_array(::std::uint32_t index, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_disable_vertex_attrib_array(index, state);
}

void canvas_native_webgl_draw_arrays(::std::uint32_t mode, ::std::int32_t first, ::std::int32_t count, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_draw_arrays(mode, first, count, state);
}

void canvas_native_webgl_draw_elements(::std::uint32_t mode, ::std::int32_t count, ::std::uint32_t element_type, ::rust::isize offset, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_draw_elements(mode, count, element_type, offset, state);
}

void canvas_native_webgl_enable(::std::uint32_t cap, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_enable(cap, state);
}

void canvas_native_webgl_enable_vertex_attrib_array(::std::uint32_t index, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_enable_vertex_attrib_array(index, state);
}

void canvas_native_webgl_finish(::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_finish(state);
}

void canvas_native_webgl_flush(::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_flush(state);
}

void canvas_native_webgl_framebuffer_renderbuffer(::std::uint32_t target, ::std::uint32_t attachment, ::std::uint32_t renderbuffertarget, ::std::uint32_t renderbuffer, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_framebuffer_renderbuffer(target, attachment, renderbuffertarget, renderbuffer, state);
}

void canvas_native_webgl_framebuffer_texture2d(::std::uint32_t target, ::std::uint32_t attachment, ::std::uint32_t textarget, ::std::uint32_t texture, ::std::int32_t level, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_framebuffer_texture2d(target, attachment, textarget, texture, level, state);
}

void canvas_native_webgl_front_face(::std::uint32_t mode, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_front_face(mode, state);
}

void canvas_native_webgl_generate_mipmap(::std::uint32_t target, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_generate_mipmap(target, state);
}

::rust::Box<::org::nativescript::canvas::WebGLActiveInfo> canvas_native_webgl_get_active_attrib(::std::uint32_t program, ::std::uint32_t index, ::org::nativescript::canvas::WebGLState &state) noexcept {
  return ::rust::Box<::org::nativescript::canvas::WebGLActiveInfo>::from_raw(org$nativescript$canvas$cxxbridge1$canvas_native_webgl_get_active_attrib(program, index, state));
}

::rust::Box<::org::nativescript::canvas::WebGLActiveInfo> canvas_native_webgl_get_active_uniform(::std::uint32_t program, ::std::uint32_t index, ::org::nativescript::canvas::WebGLState &state) noexcept {
  return ::rust::Box<::org::nativescript::canvas::WebGLActiveInfo>::from_raw(org$nativescript$canvas$cxxbridge1$canvas_native_webgl_get_active_uniform(program, index, state));
}

::rust::Vec<::std::uint32_t> canvas_native_webgl_get_attached_shaders(::std::uint32_t program, ::org::nativescript::canvas::WebGLState &state) noexcept {
  ::rust::MaybeUninit<::rust::Vec<::std::uint32_t>> return$;
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_get_attached_shaders(program, state, &return$.value);
  return ::std::move(return$.value);
}

::std::int32_t canvas_native_webgl_get_attrib_location(::std::uint32_t program, ::rust::Str name, ::org::nativescript::canvas::WebGLState &state) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_webgl_get_attrib_location(program, name, state);
}

::std::int32_t canvas_native_webgl_get_buffer_parameter(::std::uint32_t target, ::std::uint32_t pname, ::org::nativescript::canvas::WebGLState &state) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_webgl_get_buffer_parameter(target, pname, state);
}

::rust::Box<::org::nativescript::canvas::ContextAttributes> canvas_native_webgl_get_context_attributes(::org::nativescript::canvas::WebGLState &state) noexcept {
  return ::rust::Box<::org::nativescript::canvas::ContextAttributes>::from_raw(org$nativescript$canvas$cxxbridge1$canvas_native_webgl_get_context_attributes(state));
}

::std::uint32_t canvas_native_webgl_get_error(::org::nativescript::canvas::WebGLState &state) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_webgl_get_error(state);
}

::rust::Box<::org::nativescript::canvas::WebGLExtension> canvas_native_webgl_get_extension(::rust::Str name, ::org::nativescript::canvas::WebGLState &state) noexcept {
  return ::rust::Box<::org::nativescript::canvas::WebGLExtension>::from_raw(org$nativescript$canvas$cxxbridge1$canvas_native_webgl_get_extension(name, state));
}

::rust::Box<::org::nativescript::canvas::WebGLFramebufferAttachmentParameter> canvas_native_webgl_get_framebuffer_attachment_parameter(::std::uint32_t target, ::std::uint32_t attachment, ::std::uint32_t pname, ::org::nativescript::canvas::WebGLState &state) noexcept {
  return ::rust::Box<::org::nativescript::canvas::WebGLFramebufferAttachmentParameter>::from_raw(org$nativescript$canvas$cxxbridge1$canvas_native_webgl_get_framebuffer_attachment_parameter(target, attachment, pname, state));
}

::rust::Box<::org::nativescript::canvas::WebGLResult> canvas_native_webgl_get_parameter(::std::uint32_t pname, ::org::nativescript::canvas::WebGLState &state) noexcept {
  return ::rust::Box<::org::nativescript::canvas::WebGLResult>::from_raw(org$nativescript$canvas$cxxbridge1$canvas_native_webgl_get_parameter(pname, state));
}

::rust::String canvas_native_webgl_get_program_info_log(::std::uint32_t program, ::org::nativescript::canvas::WebGLState &state) noexcept {
  ::rust::MaybeUninit<::rust::String> return$;
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_get_program_info_log(program, state, &return$.value);
  return ::std::move(return$.value);
}

::rust::Box<::org::nativescript::canvas::WebGLResult> canvas_native_webgl_get_program_parameter(::std::uint32_t program, ::std::uint32_t pname, ::org::nativescript::canvas::WebGLState &state) noexcept {
  return ::rust::Box<::org::nativescript::canvas::WebGLResult>::from_raw(org$nativescript$canvas$cxxbridge1$canvas_native_webgl_get_program_parameter(program, pname, state));
}

::std::int32_t canvas_native_webgl_get_renderbuffer_parameter(::std::uint32_t target, ::std::uint32_t pname, ::org::nativescript::canvas::WebGLState &state) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_webgl_get_renderbuffer_parameter(target, pname, state);
}

::rust::String canvas_native_webgl_get_shader_info_log(::std::uint32_t shader, ::org::nativescript::canvas::WebGLState &state) noexcept {
  ::rust::MaybeUninit<::rust::String> return$;
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_get_shader_info_log(shader, state, &return$.value);
  return ::std::move(return$.value);
}

::rust::Box<::org::nativescript::canvas::WebGLResult> canvas_native_webgl_get_shader_parameter(::std::uint32_t shader, ::std::uint32_t pname, ::org::nativescript::canvas::WebGLState &state) noexcept {
  return ::rust::Box<::org::nativescript::canvas::WebGLResult>::from_raw(org$nativescript$canvas$cxxbridge1$canvas_native_webgl_get_shader_parameter(shader, pname, state));
}

::rust::Box<::org::nativescript::canvas::WebGLShaderPrecisionFormat> canvas_native_webgl_get_shader_precision_format(::std::uint32_t shader_type, ::std::uint32_t precision_type, ::org::nativescript::canvas::WebGLState &state) noexcept {
  return ::rust::Box<::org::nativescript::canvas::WebGLShaderPrecisionFormat>::from_raw(org$nativescript$canvas$cxxbridge1$canvas_native_webgl_get_shader_precision_format(shader_type, precision_type, state));
}

::rust::String canvas_native_webgl_get_shader_source(::std::uint32_t shader, ::org::nativescript::canvas::WebGLState &state) noexcept {
  ::rust::MaybeUninit<::rust::String> return$;
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_get_shader_source(shader, state, &return$.value);
  return ::std::move(return$.value);
}

::rust::Vec<::rust::String> canvas_native_webgl_get_supported_extensions(::org::nativescript::canvas::WebGLState &state) noexcept {
  ::rust::MaybeUninit<::rust::Vec<::rust::String>> return$;
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_get_supported_extensions(state, &return$.value);
  return ::std::move(return$.value);
}

::rust::String canvas_native_webgl_get_supported_extensions_to_string(::org::nativescript::canvas::WebGLState &state) noexcept {
  ::rust::MaybeUninit<::rust::String> return$;
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_get_supported_extensions_to_string(state, &return$.value);
  return ::std::move(return$.value);
}

::std::int32_t canvas_native_webgl_get_tex_parameter(::std::uint32_t target, ::std::uint32_t pname, ::org::nativescript::canvas::WebGLState &state) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_webgl_get_tex_parameter(target, pname, state);
}

::std::int32_t canvas_native_webgl_get_uniform_location(::std::uint32_t program, ::rust::Str name, ::org::nativescript::canvas::WebGLState &state) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_webgl_get_uniform_location(program, name, state);
}

::rust::Box<::org::nativescript::canvas::WebGLResult> canvas_native_webgl_get_uniform(::std::uint32_t program, ::std::int32_t location, ::org::nativescript::canvas::WebGLState &state) noexcept {
  return ::rust::Box<::org::nativescript::canvas::WebGLResult>::from_raw(org$nativescript$canvas$cxxbridge1$canvas_native_webgl_get_uniform(program, location, state));
}

::std::size_t canvas_native_webgl_get_vertex_attrib_offset(::std::uint32_t index, ::std::uint32_t pname, ::org::nativescript::canvas::WebGLState &state) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_webgl_get_vertex_attrib_offset(index, pname, state);
}

::rust::Box<::org::nativescript::canvas::WebGLResult> canvas_native_webgl_get_vertex_attrib(::std::uint32_t index, ::std::uint32_t pname, ::org::nativescript::canvas::WebGLState &state) noexcept {
  return ::rust::Box<::org::nativescript::canvas::WebGLResult>::from_raw(org$nativescript$canvas$cxxbridge1$canvas_native_webgl_get_vertex_attrib(index, pname, state));
}

bool canvas_native_webgl_get_is_context_lost(::org::nativescript::canvas::WebGLState &state) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_webgl_get_is_context_lost(state);
}

void canvas_native_webgl_hint(::std::uint32_t target, ::std::uint32_t mode, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_hint(target, mode, state);
}

bool canvas_native_webgl_is_buffer(::std::uint32_t buffer, ::org::nativescript::canvas::WebGLState &state) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_webgl_is_buffer(buffer, state);
}

bool canvas_native_webgl_is_enabled(::std::uint32_t cap, ::org::nativescript::canvas::WebGLState &state) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_webgl_is_enabled(cap, state);
}

bool canvas_native_webgl_is_framebuffer(::std::uint32_t framebuffer, ::org::nativescript::canvas::WebGLState &state) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_webgl_is_framebuffer(framebuffer, state);
}

bool canvas_native_webgl_is_program(::std::uint32_t program, ::org::nativescript::canvas::WebGLState &state) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_webgl_is_program(program, state);
}

bool canvas_native_webgl_is_renderbuffer(::std::uint32_t renderbuffer, ::org::nativescript::canvas::WebGLState &state) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_webgl_is_renderbuffer(renderbuffer, state);
}

bool canvas_native_webgl_is_shader(::std::uint32_t shader, ::org::nativescript::canvas::WebGLState &state) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_webgl_is_shader(shader, state);
}

bool canvas_native_webgl_is_texture(::std::uint32_t texture, ::org::nativescript::canvas::WebGLState &state) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_webgl_is_texture(texture, state);
}

void canvas_native_webgl_line_width(float width, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_line_width(width, state);
}

void canvas_native_webgl_link_program(::std::uint32_t program, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_link_program(program, state);
}

void canvas_native_webgl_pixel_storei(::std::uint32_t pname, ::std::int32_t param, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_pixel_storei(pname, param, state);
}

void canvas_native_webgl_polygon_offset(float factor, float units, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_polygon_offset(factor, units, state);
}

void canvas_native_webgl_read_pixels_u8(::std::int32_t x, ::std::int32_t y, ::std::int32_t width, ::std::int32_t height, ::std::uint32_t format, ::std::uint32_t pixel_type, ::rust::Slice<::std::uint8_t > pixels, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_read_pixels_u8(x, y, width, height, format, pixel_type, pixels, state);
}

void canvas_native_webgl_read_pixels_u16(::std::int32_t x, ::std::int32_t y, ::std::int32_t width, ::std::int32_t height, ::std::uint32_t format, ::std::uint32_t pixel_type, ::rust::Slice<::std::uint16_t > pixels, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_read_pixels_u16(x, y, width, height, format, pixel_type, pixels, state);
}

void canvas_native_webgl_read_pixels_f32(::std::int32_t x, ::std::int32_t y, ::std::int32_t width, ::std::int32_t height, ::std::uint32_t format, ::std::uint32_t pixel_type, ::rust::Slice<float > pixels, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_read_pixels_f32(x, y, width, height, format, pixel_type, pixels, state);
}

void canvas_native_webgl_renderbuffer_storage(::std::uint32_t target, ::std::uint32_t internal_format, ::std::int32_t width, ::std::int32_t height, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_renderbuffer_storage(target, internal_format, width, height, state);
}

void canvas_native_webgl_sample_coverage(float value, bool invert, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_sample_coverage(value, invert, state);
}

void canvas_native_webgl_scissor(::std::int32_t x, ::std::int32_t y, ::std::int32_t width, ::std::int32_t height, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_scissor(x, y, width, height, state);
}

void canvas_native_webgl_shader_source(::std::uint32_t shader, ::rust::Str source, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_shader_source(shader, source, state);
}

void canvas_native_webgl_stencil_func(::std::uint32_t func, ::std::int32_t reference, ::std::uint32_t mask, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_stencil_func(func, reference, mask, state);
}

void canvas_native_webgl_stencil_func_separate(::std::uint32_t face, ::std::uint32_t func, ::std::int32_t reference, ::std::uint32_t mask, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_stencil_func_separate(face, func, reference, mask, state);
}

void canvas_native_webgl_stencil_mask(::std::uint32_t mask, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_stencil_mask(mask, state);
}

void canvas_native_webgl_stencil_mask_separate(::std::uint32_t face, ::std::uint32_t mask, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_stencil_mask_separate(face, mask, state);
}

void canvas_native_webgl_stencil_op(::std::uint32_t fail, ::std::uint32_t zfail, ::std::uint32_t zpass, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_stencil_op(fail, zfail, zpass, state);
}

void canvas_native_webgl_stencil_op_separate(::std::uint32_t face, ::std::uint32_t fail, ::std::uint32_t zfail, ::std::uint32_t zpass, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_stencil_op_separate(face, fail, zfail, zpass, state);
}

void canvas_native_webgl_tex_image2d_image_none(::std::int32_t target, ::std::int32_t level, ::std::int32_t internalformat, ::std::int32_t format, ::std::int32_t image_type, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_tex_image2d_image_none(target, level, internalformat, format, image_type, state);
}

void canvas_native_webgl_tex_image2d_canvas2d(::std::int32_t target, ::std::int32_t level, ::std::int32_t internalformat, ::std::int32_t format, ::std::int32_t image_type, ::org::nativescript::canvas::CanvasRenderingContext2D &canvas, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_tex_image2d_canvas2d(target, level, internalformat, format, image_type, canvas, state);
}

void canvas_native_webgl_tex_image2d_webgl(::std::int32_t target, ::std::int32_t level, ::std::int32_t internalformat, ::std::int32_t format, ::std::int32_t image_type, ::org::nativescript::canvas::WebGLState &webgl, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_tex_image2d_webgl(target, level, internalformat, format, image_type, webgl, state);
}

void canvas_native_webgl_tex_image2d(::std::int32_t target, ::std::int32_t level, ::std::int32_t internalformat, ::std::int32_t width, ::std::int32_t height, ::std::int32_t border, ::std::int32_t format, ::std::int32_t image_type, ::rust::Slice<::std::uint8_t > buf, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_tex_image2d(target, level, internalformat, width, height, border, format, image_type, buf, state);
}

void canvas_native_webgl_tex_image2d_none(::std::int32_t target, ::std::int32_t level, ::std::int32_t internalformat, ::std::int32_t width, ::std::int32_t height, ::std::int32_t border, ::std::int32_t format, ::std::int32_t image_type, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_tex_image2d_none(target, level, internalformat, width, height, border, format, image_type, state);
}

void canvas_native_webgl_tex_image2d_image_asset(::std::int32_t target, ::std::int32_t level, ::std::int32_t internalformat, ::std::int32_t format, ::std::int32_t image_type, ::org::nativescript::canvas::ImageAsset &image_asset, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_tex_image2d_image_asset(target, level, internalformat, format, image_type, image_asset, state);
}

void canvas_native_webgl_tex_parameterf(::std::uint32_t target, ::std::uint32_t pname, float param, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_tex_parameterf(target, pname, param, state);
}

void canvas_native_webgl_tex_parameteri(::std::uint32_t target, ::std::uint32_t pname, ::std::int32_t param, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_tex_parameteri(target, pname, param, state);
}

void canvas_native_webgl_tex_sub_image2d_asset(::std::uint32_t target, ::std::int32_t level, ::std::int32_t xoffset, ::std::int32_t yoffset, ::std::uint32_t format, ::std::int32_t image_type, ::org::nativescript::canvas::ImageAsset &asset, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_tex_sub_image2d_asset(target, level, xoffset, yoffset, format, image_type, asset, state);
}

void canvas_native_webgl_tex_sub_image2d_canvas2d(::std::uint32_t target, ::std::int32_t level, ::std::int32_t xoffset, ::std::int32_t yoffset, ::std::uint32_t format, ::std::int32_t image_type, ::org::nativescript::canvas::CanvasRenderingContext2D &canvas, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_tex_sub_image2d_canvas2d(target, level, xoffset, yoffset, format, image_type, canvas, state);
}

void canvas_native_webgl_tex_sub_image2d_webgl(::std::uint32_t target, ::std::int32_t level, ::std::int32_t xoffset, ::std::int32_t yoffset, ::std::uint32_t format, ::std::int32_t image_type, ::org::nativescript::canvas::WebGLState &webgl, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_tex_sub_image2d_webgl(target, level, xoffset, yoffset, format, image_type, webgl, state);
}

void canvas_native_webgl_tex_sub_image2d(::std::uint32_t target, ::std::int32_t level, ::std::int32_t xoffset, ::std::int32_t yoffset, ::std::int32_t width, ::std::int32_t height, ::std::uint32_t format, ::std::int32_t image_type, ::rust::Slice<::std::uint8_t const> buf, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_tex_sub_image2d(target, level, xoffset, yoffset, width, height, format, image_type, buf, state);
}

void canvas_native_webgl_uniform1f(::std::int32_t location, float v0, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_uniform1f(location, v0, state);
}

void canvas_native_webgl_uniform1fv(::std::int32_t location, ::rust::Slice<float const> value, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_uniform1fv(location, value, state);
}

void canvas_native_webgl_uniform1i(::std::int32_t location, ::std::int32_t v0, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_uniform1i(location, v0, state);
}

void canvas_native_webgl_uniform1iv(::std::int32_t location, ::rust::Slice<::std::int32_t const> value, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_uniform1iv(location, value, state);
}

void canvas_native_webgl_uniform2f(::std::int32_t location, float v0, float v1, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_uniform2f(location, v0, v1, state);
}

void canvas_native_webgl_uniform2fv(::std::int32_t location, ::rust::Slice<float const> value, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_uniform2fv(location, value, state);
}

void canvas_native_webgl_uniform2i(::std::int32_t location, ::std::int32_t v0, ::std::int32_t v1, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_uniform2i(location, v0, v1, state);
}

void canvas_native_webgl_uniform2iv(::std::int32_t location, ::rust::Slice<::std::int32_t const> value, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_uniform2iv(location, value, state);
}

void canvas_native_webgl_uniform3f(::std::int32_t location, float v0, float v1, float v2, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_uniform3f(location, v0, v1, v2, state);
}

void canvas_native_webgl_uniform3fv(::std::int32_t location, ::rust::Slice<float const> value, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_uniform3fv(location, value, state);
}

void canvas_native_webgl_uniform3i(::std::int32_t location, ::std::int32_t v0, ::std::int32_t v1, ::std::int32_t v2, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_uniform3i(location, v0, v1, v2, state);
}

void canvas_native_webgl_uniform3iv(::std::int32_t location, ::rust::Slice<::std::int32_t const> value, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_uniform3iv(location, value, state);
}

void canvas_native_webgl_uniform4f(::std::int32_t location, float v0, float v1, float v2, float v3, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_uniform4f(location, v0, v1, v2, v3, state);
}

void canvas_native_webgl_uniform4fv(::std::int32_t location, ::rust::Slice<float const> value, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_uniform4fv(location, value, state);
}

void canvas_native_webgl_uniform4i(::std::int32_t location, ::std::int32_t v0, ::std::int32_t v1, ::std::int32_t v2, ::std::int32_t v3, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_uniform4i(location, v0, v1, v2, v3, state);
}

void canvas_native_webgl_uniform4iv(::std::int32_t location, ::rust::Slice<::std::int32_t const> value, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_uniform4iv(location, value, state);
}

void canvas_native_webgl_uniform_matrix2fv(::std::int32_t location, bool transpose, ::rust::Slice<float const> value, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_uniform_matrix2fv(location, transpose, value, state);
}

void canvas_native_webgl_uniform_matrix3fv(::std::int32_t location, bool transpose, ::rust::Slice<float const> value, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_uniform_matrix3fv(location, transpose, value, state);
}

void canvas_native_webgl_uniform_matrix4fv(::std::int32_t location, bool transpose, ::rust::Slice<float const> value, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_uniform_matrix4fv(location, transpose, value, state);
}

void canvas_native_webgl_use_program(::std::uint32_t program, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_use_program(program, state);
}

void canvas_native_webgl_validate_program(::std::uint32_t program, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_validate_program(program, state);
}

void canvas_native_webgl_vertex_attrib1f(::std::uint32_t index, float v0, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_vertex_attrib1f(index, v0, state);
}

void canvas_native_webgl_vertex_attrib1fv(::std::uint32_t index, ::rust::Slice<float const> value, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_vertex_attrib1fv(index, value, state);
}

void canvas_native_webgl_vertex_attrib2f(::std::uint32_t index, float v0, float v1, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_vertex_attrib2f(index, v0, v1, state);
}

void canvas_native_webgl_vertex_attrib2fv(::std::uint32_t index, ::rust::Slice<float const> value, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_vertex_attrib2fv(index, value, state);
}

void canvas_native_webgl_vertex_attrib3f(::std::uint32_t index, float v0, float v1, float v2, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_vertex_attrib3f(index, v0, v1, v2, state);
}

void canvas_native_webgl_vertex_attrib3fv(::std::uint32_t index, ::rust::Slice<float const> value, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_vertex_attrib3fv(index, value, state);
}

void canvas_native_webgl_vertex_attrib4f(::std::uint32_t index, float v0, float v1, float v2, float v3, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_vertex_attrib4f(index, v0, v1, v2, v3, state);
}

void canvas_native_webgl_vertex_attrib4fv(::std::uint32_t index, ::rust::Slice<float const> value, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_vertex_attrib4fv(index, value, state);
}

void canvas_native_webgl_vertex_attrib_pointer(::std::uint32_t index, ::std::int32_t size, ::std::uint32_t d_type, bool normalized, ::std::int32_t stride, ::rust::isize offset, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_vertex_attrib_pointer(index, size, d_type, normalized, stride, offset, state);
}

void canvas_native_webgl_viewport(::std::int32_t x, ::std::int32_t y, ::std::int32_t width, ::std::int32_t height, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl_viewport(x, y, width, height, state);
}

::std::size_t WebGLSync::layout::size() noexcept {
  return org$nativescript$canvas$cxxbridge1$WebGLSync$operator$sizeof();
}

::std::size_t WebGLSync::layout::align() noexcept {
  return org$nativescript$canvas$cxxbridge1$WebGLSync$operator$alignof();
}

::std::size_t WebGLIndexedParameter::layout::size() noexcept {
  return org$nativescript$canvas$cxxbridge1$WebGLIndexedParameter$operator$sizeof();
}

::std::size_t WebGLIndexedParameter::layout::align() noexcept {
  return org$nativescript$canvas$cxxbridge1$WebGLIndexedParameter$operator$alignof();
}

::rust::isize canvas_native_webgl2_indexed_parameter_get_value(::org::nativescript::canvas::WebGLIndexedParameter const &param) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_indexed_parameter_get_value(param);
}

::rust::isize canvas_native_webgl2_indexed_parameter_get_buffer_value(::org::nativescript::canvas::WebGLIndexedParameter const &param) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_indexed_parameter_get_buffer_value(param);
}

bool canvas_native_webgl2_indexed_parameter_get_is_buffer(::org::nativescript::canvas::WebGLIndexedParameter const &param) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_indexed_parameter_get_is_buffer(param);
}

void canvas_native_webgl2_begin_query(::std::uint32_t target, ::std::uint32_t id, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_begin_query(target, id, state);
}

void canvas_native_webgl2_begin_transform_feedback(::std::uint32_t primitive_mode, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_begin_transform_feedback(primitive_mode, state);
}

void canvas_native_webgl2_bind_buffer_base(::std::uint32_t target, ::std::uint32_t index, ::std::uint32_t buffer, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_bind_buffer_base(target, index, buffer, state);
}

void canvas_native_webgl2_bind_buffer_range(::std::uint32_t target, ::std::uint32_t index, ::std::uint32_t buffer, ::rust::isize offset, ::rust::isize size, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_bind_buffer_range(target, index, buffer, offset, size, state);
}

void canvas_native_webgl2_bind_sampler(::std::uint32_t unit, ::std::uint32_t sampler, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_bind_sampler(unit, sampler, state);
}

void canvas_native_webgl2_bind_transform_feedback(::std::uint32_t target, ::std::uint32_t transform_feedback, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_bind_transform_feedback(target, transform_feedback, state);
}

void canvas_native_webgl2_bind_vertex_array(::std::uint32_t vertex_array, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_bind_vertex_array(vertex_array, state);
}

void canvas_native_webgl2_blit_framebuffer(::std::int32_t src_x0, ::std::int32_t src_y0, ::std::int32_t src_x1, ::std::int32_t src_y1, ::std::int32_t dst_x0, ::std::int32_t dst_y0, ::std::int32_t dst_x1, ::std::int32_t dst_y1, ::std::uint32_t mask, ::std::uint32_t filter, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_blit_framebuffer(src_x0, src_y0, src_x1, src_y1, dst_x0, dst_y0, dst_x1, dst_y1, mask, filter, state);
}

void canvas_native_webgl2_clear_bufferfi(::std::uint32_t buffer, ::std::int32_t drawbuffer, float depth, ::std::int32_t stencil, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_clear_bufferfi(buffer, drawbuffer, depth, stencil, state);
}

void canvas_native_webgl2_clear_bufferfv(::std::uint32_t buffer, ::std::int32_t drawbuffer, ::rust::Slice<float const> values, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_clear_bufferfv(buffer, drawbuffer, values, state);
}

void canvas_native_webgl2_clear_bufferiv(::std::uint32_t buffer, ::std::int32_t drawbuffer, ::rust::Slice<::std::int32_t const> values, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_clear_bufferiv(buffer, drawbuffer, values, state);
}

void canvas_native_webgl2_clear_bufferuiv(::std::uint32_t buffer, ::std::int32_t drawbuffer, ::rust::Slice<::std::uint32_t const> values, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_clear_bufferuiv(buffer, drawbuffer, values, state);
}

::std::uint32_t canvas_native_webgl2_client_wait_sync(::org::nativescript::canvas::WebGLSync const &sync, ::std::uint32_t flags, ::rust::isize timeout, ::org::nativescript::canvas::WebGLState &state) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_client_wait_sync(sync, flags, timeout, state);
}

void canvas_native_webgl2_compressed_tex_sub_image3d_none(::std::uint32_t target, ::std::int32_t level, ::std::int32_t xoffset, ::std::int32_t yoffset, ::std::int32_t zoffset, ::std::int32_t width, ::std::int32_t height, ::std::int32_t depth, ::std::uint32_t format, ::std::int32_t image_size, ::std::int32_t offset, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_compressed_tex_sub_image3d_none(target, level, xoffset, yoffset, zoffset, width, height, depth, format, image_size, offset, state);
}

void canvas_native_webgl2_compressed_tex_sub_image3d(::std::uint32_t target, ::std::int32_t level, ::std::int32_t xoffset, ::std::int32_t yoffset, ::std::int32_t zoffset, ::std::int32_t width, ::std::int32_t height, ::std::int32_t depth, ::std::uint32_t format, ::rust::Slice<::std::uint8_t const> src, ::std::size_t src_offset, ::std::size_t src_length_override, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_compressed_tex_sub_image3d(target, level, xoffset, yoffset, zoffset, width, height, depth, format, src, src_offset, src_length_override, state);
}

void canvas_native_webgl2_copy_buffer_sub_data(::std::uint32_t read_target, ::std::uint32_t write_target, ::rust::isize read_offset, ::rust::isize write_offset, ::rust::isize size, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_copy_buffer_sub_data(read_target, write_target, read_offset, write_offset, size, state);
}

void canvas_native_webgl2_copy_tex_sub_image3d(::std::uint32_t target, ::std::int32_t level, ::std::int32_t xoffset, ::std::int32_t yoffset, ::std::int32_t zoffset, ::std::int32_t x, ::std::int32_t y, ::std::int32_t width, ::std::int32_t height, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_copy_tex_sub_image3d(target, level, xoffset, yoffset, zoffset, x, y, width, height, state);
}

::std::uint32_t canvas_native_webgl2_create_query(::org::nativescript::canvas::WebGLState &state) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_create_query(state);
}

::std::uint32_t canvas_native_webgl2_create_sampler(::org::nativescript::canvas::WebGLState &state) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_create_sampler(state);
}

::std::uint32_t canvas_native_webgl2_create_transform_feedback(::org::nativescript::canvas::WebGLState &state) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_create_transform_feedback(state);
}

::std::uint32_t canvas_native_webgl2_create_vertex_array(::org::nativescript::canvas::WebGLState &state) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_create_vertex_array(state);
}

void canvas_native_webgl2_delete_query_with_query(::std::uint32_t id, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_delete_query_with_query(id, state);
}

void canvas_native_webgl2_delete_sampler_with_sampler(::std::uint32_t sampler, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_delete_sampler_with_sampler(sampler, state);
}

void canvas_native_webgl2_delete_sync_with_sync(::org::nativescript::canvas::WebGLSync const &sync, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_delete_sync_with_sync(sync, state);
}

void canvas_native_webgl2_delete_transform_feedback(::std::uint32_t transform_feedback, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_delete_transform_feedback(transform_feedback, state);
}

void canvas_native_webgl2_delete_vertex_array_with_vertex_array(::std::uint32_t vertex_array, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_delete_vertex_array_with_vertex_array(vertex_array, state);
}

void canvas_native_webgl2_draw_arrays_instanced(::std::uint32_t mode, ::std::int32_t first, ::std::int32_t count, ::std::int32_t instance_count, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_draw_arrays_instanced(mode, first, count, instance_count, state);
}

void canvas_native_webgl2_draw_buffers(::rust::Slice<::std::uint32_t const> buffers, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_draw_buffers(buffers, state);
}

void canvas_native_webgl2_draw_elements_instanced(::std::uint32_t mode, ::std::int32_t count, ::std::uint32_t type_, ::rust::isize offset, ::std::int32_t instance_count, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_draw_elements_instanced(mode, count, type_, offset, instance_count, state);
}

void canvas_native_webgl2_draw_range_elements(::std::uint32_t mode, ::std::uint32_t start, ::std::uint32_t end, ::std::int32_t count, ::std::uint32_t type_, ::rust::isize offset, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_draw_range_elements(mode, start, end, count, type_, offset, state);
}

void canvas_native_webgl2_end_query(::std::uint32_t target, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_end_query(target, state);
}

void canvas_native_webgl2_end_transform_feedback(::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_end_transform_feedback(state);
}

::rust::Box<::org::nativescript::canvas::WebGLSync> canvas_native_webgl2_fence_sync(::std::uint32_t condition, ::std::uint32_t flags, ::org::nativescript::canvas::WebGLState &state) noexcept {
  return ::rust::Box<::org::nativescript::canvas::WebGLSync>::from_raw(org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_fence_sync(condition, flags, state));
}

void canvas_native_webgl2_framebuffer_texture_layer(::std::uint32_t target, ::std::uint32_t attachment, ::std::uint32_t texture, ::std::int32_t level, ::std::int32_t layer, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_framebuffer_texture_layer(target, attachment, texture, level, layer, state);
}

::rust::String canvas_native_webgl2_get_active_uniform_block_name(::std::uint32_t program, ::std::uint32_t uniform_block_index, ::org::nativescript::canvas::WebGLState &state) noexcept {
  ::rust::MaybeUninit<::rust::String> return$;
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_get_active_uniform_block_name(program, uniform_block_index, state, &return$.value);
  return ::std::move(return$.value);
}

::rust::Box<::org::nativescript::canvas::WebGLResult> canvas_native_webgl2_get_active_uniform_block_parameter(::std::uint32_t program, ::std::uint32_t uniform_block_index, ::std::uint32_t pname, ::org::nativescript::canvas::WebGLState &state) noexcept {
  return ::rust::Box<::org::nativescript::canvas::WebGLResult>::from_raw(org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_get_active_uniform_block_parameter(program, uniform_block_index, pname, state));
}

::rust::Box<::org::nativescript::canvas::WebGLResult> canvas_native_webgl2_get_active_uniforms(::std::uint32_t program, ::rust::Slice<::std::uint32_t const> uniform_indices, ::std::uint32_t pname, ::org::nativescript::canvas::WebGLState &state) noexcept {
  return ::rust::Box<::org::nativescript::canvas::WebGLResult>::from_raw(org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_get_active_uniforms(program, uniform_indices, pname, state));
}

void canvas_native_webgl2_get_buffer_sub_data(::std::uint32_t target, ::rust::isize src_byte_offset, ::rust::Slice<::std::uint8_t > dst_data, ::std::size_t dst_offset, ::std::size_t length, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_get_buffer_sub_data(target, src_byte_offset, dst_data, dst_offset, length, state);
}

::std::int32_t canvas_native_webgl2_get_frag_data_location(::std::uint32_t program, ::rust::Str name, ::org::nativescript::canvas::WebGLState &state) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_get_frag_data_location(program, name, state);
}

::rust::Box<::org::nativescript::canvas::WebGLIndexedParameter> canvas_native_webgl2_get_indexed_parameter(::std::uint32_t target, ::std::uint32_t index, ::org::nativescript::canvas::WebGLState &state) noexcept {
  return ::rust::Box<::org::nativescript::canvas::WebGLIndexedParameter>::from_raw(org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_get_indexed_parameter(target, index, state));
}

::rust::Box<::org::nativescript::canvas::WebGLResult> canvas_native_webgl2_get_internalformat_parameter(::std::uint32_t target, ::std::uint32_t internalformat, ::std::uint32_t pname, ::org::nativescript::canvas::WebGLState &state) noexcept {
  return ::rust::Box<::org::nativescript::canvas::WebGLResult>::from_raw(org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_get_internalformat_parameter(target, internalformat, pname, state));
}

::rust::Box<::org::nativescript::canvas::WebGLResult> canvas_native_webgl2_get_parameter(::std::uint32_t pname, ::org::nativescript::canvas::WebGLState &state) noexcept {
  return ::rust::Box<::org::nativescript::canvas::WebGLResult>::from_raw(org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_get_parameter(pname, state));
}

::rust::Box<::org::nativescript::canvas::WebGLResult> canvas_native_webgl2_get_query_parameter(::std::uint32_t query, ::std::uint32_t pname, ::org::nativescript::canvas::WebGLState &state) noexcept {
  return ::rust::Box<::org::nativescript::canvas::WebGLResult>::from_raw(org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_get_query_parameter(query, pname, state));
}

::rust::Box<::org::nativescript::canvas::WebGLResult> canvas_native_webgl2_get_query(::std::uint32_t target, ::std::uint32_t pname, ::org::nativescript::canvas::WebGLState &state) noexcept {
  return ::rust::Box<::org::nativescript::canvas::WebGLResult>::from_raw(org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_get_query(target, pname, state));
}

::rust::Box<::org::nativescript::canvas::WebGLResult> canvas_native_webgl2_get_sampler_parameter(::std::uint32_t sampler, ::std::uint32_t pname, ::org::nativescript::canvas::WebGLState &state) noexcept {
  return ::rust::Box<::org::nativescript::canvas::WebGLResult>::from_raw(org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_get_sampler_parameter(sampler, pname, state));
}

::rust::Box<::org::nativescript::canvas::WebGLResult> canvas_native_webgl2_get_sync_parameter(::org::nativescript::canvas::WebGLSync const &sync, ::std::uint32_t pname, ::org::nativescript::canvas::WebGLState &state) noexcept {
  return ::rust::Box<::org::nativescript::canvas::WebGLResult>::from_raw(org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_get_sync_parameter(sync, pname, state));
}

::rust::Box<::org::nativescript::canvas::WebGLActiveInfo> canvas_native_webgl2_get_transform_feedback_varying(::std::uint32_t program, ::std::uint32_t index, ::org::nativescript::canvas::WebGLState &state) noexcept {
  return ::rust::Box<::org::nativescript::canvas::WebGLActiveInfo>::from_raw(org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_get_transform_feedback_varying(program, index, state));
}

::std::uint32_t canvas_native_webgl2_get_uniform_block_index(::std::uint32_t program, ::rust::Str uniform_block_name, ::org::nativescript::canvas::WebGLState &state) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_get_uniform_block_index(program, uniform_block_name, state);
}

::rust::Vec<::std::uint32_t> canvas_native_webgl2_get_uniform_indices(::std::uint32_t program, ::rust::Slice<::rust::Str const> uniform_names, ::org::nativescript::canvas::WebGLState &state) noexcept {
  ::rust::MaybeUninit<::rust::Vec<::std::uint32_t>> return$;
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_get_uniform_indices(program, uniform_names, state, &return$.value);
  return ::std::move(return$.value);
}

void canvas_native_webgl2_invalidate_framebuffer(::std::uint32_t target, ::rust::Slice<::std::uint32_t const> attachments, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_invalidate_framebuffer(target, attachments, state);
}

void canvas_native_webgl2_invalidate_sub_framebuffer(::std::uint32_t target, ::rust::Slice<::std::uint32_t const> attachments, ::std::int32_t x, ::std::int32_t y, ::std::int32_t width, ::std::int32_t height, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_invalidate_sub_framebuffer(target, attachments, x, y, width, height, state);
}

bool canvas_native_webgl2_is_query(::std::uint32_t query, ::org::nativescript::canvas::WebGLState &state) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_is_query(query, state);
}

bool canvas_native_webgl2_is_sampler(::std::uint32_t sampler, ::org::nativescript::canvas::WebGLState &state) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_is_sampler(sampler, state);
}

bool canvas_native_webgl2_is_sync(::org::nativescript::canvas::WebGLSync const &sync, ::org::nativescript::canvas::WebGLState &state) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_is_sync(sync, state);
}

bool canvas_native_webgl2_is_transform_feedback(::std::uint32_t transform_feedback, ::org::nativescript::canvas::WebGLState &state) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_is_transform_feedback(transform_feedback, state);
}

bool canvas_native_webgl2_is_vertex_array(::std::uint32_t vertex_array, ::org::nativescript::canvas::WebGLState &state) noexcept {
  return org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_is_vertex_array(vertex_array, state);
}

void canvas_native_webgl2_pause_transform_feedback(::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_pause_transform_feedback(state);
}

void canvas_native_webgl2_read_buffer(::std::uint32_t src, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_read_buffer(src, state);
}

void canvas_native_webgl2_renderbuffer_storage_multisample(::std::uint32_t target, ::std::int32_t samples, ::std::uint32_t internal_format, ::std::int32_t width, ::std::int32_t height, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_renderbuffer_storage_multisample(target, samples, internal_format, width, height, state);
}

void canvas_native_webgl2_resume_transform_feedback(::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_resume_transform_feedback(state);
}

void canvas_native_webgl2_sampler_parameterf(::std::uint32_t sampler, ::std::uint32_t pname, float param, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_sampler_parameterf(sampler, pname, param, state);
}

void canvas_native_webgl2_sampler_parameteri(::std::uint32_t sampler, ::std::uint32_t pname, ::std::int32_t param, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_sampler_parameteri(sampler, pname, param, state);
}

void canvas_native_webgl2_tex_image3d_none(::std::uint32_t target, ::std::int32_t level, ::std::int32_t internalformat, ::std::int32_t width, ::std::int32_t height, ::std::int32_t depth, ::std::int32_t border, ::std::uint32_t format, ::std::uint32_t type_, ::std::size_t offset, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_tex_image3d_none(target, level, internalformat, width, height, depth, border, format, type_, offset, state);
}

void canvas_native_webgl2_tex_image3d_asset(::std::uint32_t target, ::std::int32_t level, ::std::int32_t internalformat, ::std::int32_t width, ::std::int32_t height, ::std::int32_t depth, ::std::int32_t border, ::std::uint32_t format, ::std::uint32_t type_, ::org::nativescript::canvas::ImageAsset const &asset, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_tex_image3d_asset(target, level, internalformat, width, height, depth, border, format, type_, asset, state);
}

void canvas_native_webgl2_tex_image3d(::std::uint32_t target, ::std::int32_t level, ::std::int32_t internalformat, ::std::int32_t width, ::std::int32_t height, ::std::int32_t depth, ::std::int32_t border, ::std::uint32_t format, ::std::uint32_t type_, ::rust::Slice<::std::uint8_t const> buf, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_tex_image3d(target, level, internalformat, width, height, depth, border, format, type_, buf, state);
}

void canvas_native_webgl2_tex_image3d_offset(::std::uint32_t target, ::std::int32_t level, ::std::int32_t internalformat, ::std::int32_t width, ::std::int32_t height, ::std::int32_t depth, ::std::int32_t border, ::std::uint32_t format, ::std::uint32_t type_, ::rust::Slice<::std::uint8_t const> buf, ::std::size_t offset, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_tex_image3d_offset(target, level, internalformat, width, height, depth, border, format, type_, buf, offset, state);
}

void canvas_native_webgl2_tex_storage2d(::std::uint32_t target, ::std::int32_t levels, ::std::uint32_t internalformat, ::std::int32_t width, ::std::int32_t height, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_tex_storage2d(target, levels, internalformat, width, height, state);
}

void canvas_native_webgl2_tex_storage3d(::std::uint32_t target, ::std::int32_t levels, ::std::uint32_t internalformat, ::std::int32_t width, ::std::int32_t height, ::std::int32_t depth, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_tex_storage3d(target, levels, internalformat, width, height, depth, state);
}

void canvas_native_webgl2_tex_sub_image3d_none(::std::uint32_t target, ::std::int32_t level, ::std::int32_t xoffset, ::std::int32_t yoffset, ::std::int32_t zoffset, ::std::int32_t width, ::std::int32_t height, ::std::int32_t depth, ::std::uint32_t format, ::std::uint32_t type_, ::std::size_t offset, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_tex_sub_image3d_none(target, level, xoffset, yoffset, zoffset, width, height, depth, format, type_, offset, state);
}

void canvas_native_webgl2_tex_sub_image3d(::std::uint32_t target, ::std::int32_t level, ::std::int32_t xoffset, ::std::int32_t yoffset, ::std::int32_t zoffset, ::std::int32_t width, ::std::int32_t height, ::std::int32_t depth, ::std::uint32_t format, ::std::uint32_t type_, ::rust::Slice<::std::uint8_t const> buf, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_tex_sub_image3d(target, level, xoffset, yoffset, zoffset, width, height, depth, format, type_, buf, state);
}

void canvas_native_webgl2_tex_sub_image3d_asset(::std::uint32_t target, ::std::int32_t level, ::std::int32_t xoffset, ::std::int32_t yoffset, ::std::int32_t zoffset, ::std::int32_t width, ::std::int32_t height, ::std::int32_t depth, ::std::uint32_t format, ::std::uint32_t type_, ::org::nativescript::canvas::ImageAsset const &asset, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_tex_sub_image3d_asset(target, level, xoffset, yoffset, zoffset, width, height, depth, format, type_, asset, state);
}

void canvas_native_webgl2_tex_sub_image3d_offset(::std::uint32_t target, ::std::int32_t level, ::std::int32_t xoffset, ::std::int32_t yoffset, ::std::int32_t zoffset, ::std::int32_t width, ::std::int32_t height, ::std::int32_t depth, ::std::uint32_t format, ::std::uint32_t type_, ::rust::Slice<::std::uint8_t const> buf, ::std::size_t offset, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_tex_sub_image3d_offset(target, level, xoffset, yoffset, zoffset, width, height, depth, format, type_, buf, offset, state);
}

void canvas_native_webgl2_transform_feedback_varyings(::std::uint32_t program, ::rust::Slice<::rust::Str const> varyings, ::std::uint32_t buffer_mode, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_transform_feedback_varyings(program, varyings, buffer_mode, state);
}

void canvas_native_webgl2_uniform1ui(::std::int32_t location, ::std::uint32_t v0, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_uniform1ui(location, v0, state);
}

void canvas_native_webgl2_uniform1uiv(::std::int32_t location, ::rust::Slice<::std::uint32_t const> data, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_uniform1uiv(location, data, state);
}

void canvas_native_webgl2_uniform2ui(::std::int32_t location, ::std::uint32_t v0, ::std::uint32_t v1, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_uniform2ui(location, v0, v1, state);
}

void canvas_native_webgl2_uniform2uiv(::std::int32_t location, ::rust::Slice<::std::uint32_t const> data, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_uniform2uiv(location, data, state);
}

void canvas_native_webgl2_uniform3ui(::std::int32_t location, ::std::uint32_t v0, ::std::uint32_t v1, ::std::uint32_t v2, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_uniform3ui(location, v0, v1, v2, state);
}

void canvas_native_webgl2_uniform3uiv(::std::int32_t location, ::rust::Slice<::std::uint32_t const> data, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_uniform3uiv(location, data, state);
}

void canvas_native_webgl2_uniform4ui(::std::int32_t location, ::std::uint32_t v0, ::std::uint32_t v1, ::std::uint32_t v2, ::std::uint32_t v3, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_uniform4ui(location, v0, v1, v2, v3, state);
}

void canvas_native_webgl2_uniform4uiv(::std::int32_t location, ::rust::Slice<::std::uint32_t const> data, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_uniform4uiv(location, data, state);
}

void canvas_native_webgl2_uniform_block_binding(::std::uint32_t program, ::std::uint32_t uniform_block_index, ::std::uint32_t uniform_block_binding, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_uniform_block_binding(program, uniform_block_index, uniform_block_binding, state);
}

void canvas_native_webgl2_uniform_matrix2x3fv(::std::int32_t location, bool transpose, ::rust::Slice<float const> data, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_uniform_matrix2x3fv(location, transpose, data, state);
}

void canvas_native_webgl2_uniform_matrix2x4fv(::std::int32_t location, bool transpose, ::rust::Slice<float const> data, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_uniform_matrix2x4fv(location, transpose, data, state);
}

void canvas_native_webgl2_uniform_matrix3x2fv(::std::int32_t location, bool transpose, ::rust::Slice<float const> data, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_uniform_matrix3x2fv(location, transpose, data, state);
}

void canvas_native_webgl2_uniform_matrix3x4fv(::std::int32_t location, bool transpose, ::rust::Slice<float const> data, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_uniform_matrix3x4fv(location, transpose, data, state);
}

void canvas_native_webgl2_uniform_matrix4x2fv(::std::int32_t location, bool transpose, ::rust::Slice<float const> data, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_uniform_matrix4x2fv(location, transpose, data, state);
}

void canvas_native_webgl2_uniform_matrix4x3fv(::std::int32_t location, bool transpose, ::rust::Slice<float const> data, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_uniform_matrix4x3fv(location, transpose, data, state);
}

void canvas_native_webgl2_vertex_attrib_divisor(::std::uint32_t index, ::std::uint32_t divisor, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_vertex_attrib_divisor(index, divisor, state);
}

void canvas_native_webgl2_vertex_attrib_i4i(::std::uint32_t index, ::std::int32_t x, ::std::int32_t y, ::std::int32_t z, ::std::int32_t w, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_vertex_attrib_i4i(index, x, y, z, w, state);
}

void canvas_native_webgl2_vertex_attrib_i4iv(::std::uint32_t index, ::rust::Slice<::std::int32_t const> value, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_vertex_attrib_i4iv(index, value, state);
}

void canvas_native_webgl2_vertex_attrib_i4ui(::std::uint32_t index, ::std::uint32_t x, ::std::uint32_t y, ::std::uint32_t z, ::std::uint32_t w, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_vertex_attrib_i4ui(index, x, y, z, w, state);
}

void canvas_native_webgl2_vertex_attrib_i4uiv(::std::uint32_t index, ::rust::Slice<::std::uint32_t const> value, ::org::nativescript::canvas::WebGLState &state) noexcept {
  org$nativescript$canvas$cxxbridge1$canvas_native_webgl2_vertex_attrib_i4uiv(index, value, state);
}
} // namespace canvas
} // namespace nativescript
} // namespace org

extern "C" {
::org::nativescript::canvas::FileHelper *cxxbridge1$box$org$nativescript$canvas$FileHelper$alloc() noexcept;
void cxxbridge1$box$org$nativescript$canvas$FileHelper$dealloc(::org::nativescript::canvas::FileHelper *) noexcept;
void cxxbridge1$box$org$nativescript$canvas$FileHelper$drop(::rust::Box<::org::nativescript::canvas::FileHelper> *ptr) noexcept;

::org::nativescript::canvas::Raf *cxxbridge1$box$org$nativescript$canvas$Raf$alloc() noexcept;
void cxxbridge1$box$org$nativescript$canvas$Raf$dealloc(::org::nativescript::canvas::Raf *) noexcept;
void cxxbridge1$box$org$nativescript$canvas$Raf$drop(::rust::Box<::org::nativescript::canvas::Raf> *ptr) noexcept;

::org::nativescript::canvas::ImageAsset *cxxbridge1$box$org$nativescript$canvas$ImageAsset$alloc() noexcept;
void cxxbridge1$box$org$nativescript$canvas$ImageAsset$dealloc(::org::nativescript::canvas::ImageAsset *) noexcept;
void cxxbridge1$box$org$nativescript$canvas$ImageAsset$drop(::rust::Box<::org::nativescript::canvas::ImageAsset> *ptr) noexcept;

::org::nativescript::canvas::Path *cxxbridge1$box$org$nativescript$canvas$Path$alloc() noexcept;
void cxxbridge1$box$org$nativescript$canvas$Path$dealloc(::org::nativescript::canvas::Path *) noexcept;
void cxxbridge1$box$org$nativescript$canvas$Path$drop(::rust::Box<::org::nativescript::canvas::Path> *ptr) noexcept;

::org::nativescript::canvas::Matrix *cxxbridge1$box$org$nativescript$canvas$Matrix$alloc() noexcept;
void cxxbridge1$box$org$nativescript$canvas$Matrix$dealloc(::org::nativescript::canvas::Matrix *) noexcept;
void cxxbridge1$box$org$nativescript$canvas$Matrix$drop(::rust::Box<::org::nativescript::canvas::Matrix> *ptr) noexcept;

::org::nativescript::canvas::ImageData *cxxbridge1$box$org$nativescript$canvas$ImageData$alloc() noexcept;
void cxxbridge1$box$org$nativescript$canvas$ImageData$dealloc(::org::nativescript::canvas::ImageData *) noexcept;
void cxxbridge1$box$org$nativescript$canvas$ImageData$drop(::rust::Box<::org::nativescript::canvas::ImageData> *ptr) noexcept;

::org::nativescript::canvas::PaintStyle *cxxbridge1$box$org$nativescript$canvas$PaintStyle$alloc() noexcept;
void cxxbridge1$box$org$nativescript$canvas$PaintStyle$dealloc(::org::nativescript::canvas::PaintStyle *) noexcept;
void cxxbridge1$box$org$nativescript$canvas$PaintStyle$drop(::rust::Box<::org::nativescript::canvas::PaintStyle> *ptr) noexcept;

::org::nativescript::canvas::TextDecoder *cxxbridge1$box$org$nativescript$canvas$TextDecoder$alloc() noexcept;
void cxxbridge1$box$org$nativescript$canvas$TextDecoder$dealloc(::org::nativescript::canvas::TextDecoder *) noexcept;
void cxxbridge1$box$org$nativescript$canvas$TextDecoder$drop(::rust::Box<::org::nativescript::canvas::TextDecoder> *ptr) noexcept;

::org::nativescript::canvas::TextEncoder *cxxbridge1$box$org$nativescript$canvas$TextEncoder$alloc() noexcept;
void cxxbridge1$box$org$nativescript$canvas$TextEncoder$dealloc(::org::nativescript::canvas::TextEncoder *) noexcept;
void cxxbridge1$box$org$nativescript$canvas$TextEncoder$drop(::rust::Box<::org::nativescript::canvas::TextEncoder> *ptr) noexcept;

::org::nativescript::canvas::CanvasRenderingContext2D *cxxbridge1$box$org$nativescript$canvas$CanvasRenderingContext2D$alloc() noexcept;
void cxxbridge1$box$org$nativescript$canvas$CanvasRenderingContext2D$dealloc(::org::nativescript::canvas::CanvasRenderingContext2D *) noexcept;
void cxxbridge1$box$org$nativescript$canvas$CanvasRenderingContext2D$drop(::rust::Box<::org::nativescript::canvas::CanvasRenderingContext2D> *ptr) noexcept;

::org::nativescript::canvas::TextMetrics *cxxbridge1$box$org$nativescript$canvas$TextMetrics$alloc() noexcept;
void cxxbridge1$box$org$nativescript$canvas$TextMetrics$dealloc(::org::nativescript::canvas::TextMetrics *) noexcept;
void cxxbridge1$box$org$nativescript$canvas$TextMetrics$drop(::rust::Box<::org::nativescript::canvas::TextMetrics> *ptr) noexcept;

::org::nativescript::canvas::WebGLExtension *cxxbridge1$box$org$nativescript$canvas$WebGLExtension$alloc() noexcept;
void cxxbridge1$box$org$nativescript$canvas$WebGLExtension$dealloc(::org::nativescript::canvas::WebGLExtension *) noexcept;
void cxxbridge1$box$org$nativescript$canvas$WebGLExtension$drop(::rust::Box<::org::nativescript::canvas::WebGLExtension> *ptr) noexcept;

::org::nativescript::canvas::EXT_disjoint_timer_query *cxxbridge1$box$org$nativescript$canvas$EXT_disjoint_timer_query$alloc() noexcept;
void cxxbridge1$box$org$nativescript$canvas$EXT_disjoint_timer_query$dealloc(::org::nativescript::canvas::EXT_disjoint_timer_query *) noexcept;
void cxxbridge1$box$org$nativescript$canvas$EXT_disjoint_timer_query$drop(::rust::Box<::org::nativescript::canvas::EXT_disjoint_timer_query> *ptr) noexcept;

::org::nativescript::canvas::ANGLE_instanced_arrays *cxxbridge1$box$org$nativescript$canvas$ANGLE_instanced_arrays$alloc() noexcept;
void cxxbridge1$box$org$nativescript$canvas$ANGLE_instanced_arrays$dealloc(::org::nativescript::canvas::ANGLE_instanced_arrays *) noexcept;
void cxxbridge1$box$org$nativescript$canvas$ANGLE_instanced_arrays$drop(::rust::Box<::org::nativescript::canvas::ANGLE_instanced_arrays> *ptr) noexcept;

::org::nativescript::canvas::WEBGL_lose_context *cxxbridge1$box$org$nativescript$canvas$WEBGL_lose_context$alloc() noexcept;
void cxxbridge1$box$org$nativescript$canvas$WEBGL_lose_context$dealloc(::org::nativescript::canvas::WEBGL_lose_context *) noexcept;
void cxxbridge1$box$org$nativescript$canvas$WEBGL_lose_context$drop(::rust::Box<::org::nativescript::canvas::WEBGL_lose_context> *ptr) noexcept;

::org::nativescript::canvas::WEBGL_draw_buffers *cxxbridge1$box$org$nativescript$canvas$WEBGL_draw_buffers$alloc() noexcept;
void cxxbridge1$box$org$nativescript$canvas$WEBGL_draw_buffers$dealloc(::org::nativescript::canvas::WEBGL_draw_buffers *) noexcept;
void cxxbridge1$box$org$nativescript$canvas$WEBGL_draw_buffers$drop(::rust::Box<::org::nativescript::canvas::WEBGL_draw_buffers> *ptr) noexcept;

::org::nativescript::canvas::OES_vertex_array_object *cxxbridge1$box$org$nativescript$canvas$OES_vertex_array_object$alloc() noexcept;
void cxxbridge1$box$org$nativescript$canvas$OES_vertex_array_object$dealloc(::org::nativescript::canvas::OES_vertex_array_object *) noexcept;
void cxxbridge1$box$org$nativescript$canvas$OES_vertex_array_object$drop(::rust::Box<::org::nativescript::canvas::OES_vertex_array_object> *ptr) noexcept;

::org::nativescript::canvas::WebGLResult *cxxbridge1$box$org$nativescript$canvas$WebGLResult$alloc() noexcept;
void cxxbridge1$box$org$nativescript$canvas$WebGLResult$dealloc(::org::nativescript::canvas::WebGLResult *) noexcept;
void cxxbridge1$box$org$nativescript$canvas$WebGLResult$drop(::rust::Box<::org::nativescript::canvas::WebGLResult> *ptr) noexcept;

::org::nativescript::canvas::WebGLState *cxxbridge1$box$org$nativescript$canvas$WebGLState$alloc() noexcept;
void cxxbridge1$box$org$nativescript$canvas$WebGLState$dealloc(::org::nativescript::canvas::WebGLState *) noexcept;
void cxxbridge1$box$org$nativescript$canvas$WebGLState$drop(::rust::Box<::org::nativescript::canvas::WebGLState> *ptr) noexcept;

::org::nativescript::canvas::WebGLActiveInfo *cxxbridge1$box$org$nativescript$canvas$WebGLActiveInfo$alloc() noexcept;
void cxxbridge1$box$org$nativescript$canvas$WebGLActiveInfo$dealloc(::org::nativescript::canvas::WebGLActiveInfo *) noexcept;
void cxxbridge1$box$org$nativescript$canvas$WebGLActiveInfo$drop(::rust::Box<::org::nativescript::canvas::WebGLActiveInfo> *ptr) noexcept;

::org::nativescript::canvas::ContextAttributes *cxxbridge1$box$org$nativescript$canvas$ContextAttributes$alloc() noexcept;
void cxxbridge1$box$org$nativescript$canvas$ContextAttributes$dealloc(::org::nativescript::canvas::ContextAttributes *) noexcept;
void cxxbridge1$box$org$nativescript$canvas$ContextAttributes$drop(::rust::Box<::org::nativescript::canvas::ContextAttributes> *ptr) noexcept;

::org::nativescript::canvas::WebGLFramebufferAttachmentParameter *cxxbridge1$box$org$nativescript$canvas$WebGLFramebufferAttachmentParameter$alloc() noexcept;
void cxxbridge1$box$org$nativescript$canvas$WebGLFramebufferAttachmentParameter$dealloc(::org::nativescript::canvas::WebGLFramebufferAttachmentParameter *) noexcept;
void cxxbridge1$box$org$nativescript$canvas$WebGLFramebufferAttachmentParameter$drop(::rust::Box<::org::nativescript::canvas::WebGLFramebufferAttachmentParameter> *ptr) noexcept;

::org::nativescript::canvas::WebGLShaderPrecisionFormat *cxxbridge1$box$org$nativescript$canvas$WebGLShaderPrecisionFormat$alloc() noexcept;
void cxxbridge1$box$org$nativescript$canvas$WebGLShaderPrecisionFormat$dealloc(::org::nativescript::canvas::WebGLShaderPrecisionFormat *) noexcept;
void cxxbridge1$box$org$nativescript$canvas$WebGLShaderPrecisionFormat$drop(::rust::Box<::org::nativescript::canvas::WebGLShaderPrecisionFormat> *ptr) noexcept;

::org::nativescript::canvas::WebGLSync *cxxbridge1$box$org$nativescript$canvas$WebGLSync$alloc() noexcept;
void cxxbridge1$box$org$nativescript$canvas$WebGLSync$dealloc(::org::nativescript::canvas::WebGLSync *) noexcept;
void cxxbridge1$box$org$nativescript$canvas$WebGLSync$drop(::rust::Box<::org::nativescript::canvas::WebGLSync> *ptr) noexcept;

::org::nativescript::canvas::WebGLIndexedParameter *cxxbridge1$box$org$nativescript$canvas$WebGLIndexedParameter$alloc() noexcept;
void cxxbridge1$box$org$nativescript$canvas$WebGLIndexedParameter$dealloc(::org::nativescript::canvas::WebGLIndexedParameter *) noexcept;
void cxxbridge1$box$org$nativescript$canvas$WebGLIndexedParameter$drop(::rust::Box<::org::nativescript::canvas::WebGLIndexedParameter> *ptr) noexcept;
} // extern "C"

namespace rust {
inline namespace cxxbridge1 {
template <>
::org::nativescript::canvas::FileHelper *Box<::org::nativescript::canvas::FileHelper>::allocation::alloc() noexcept {
  return cxxbridge1$box$org$nativescript$canvas$FileHelper$alloc();
}
template <>
void Box<::org::nativescript::canvas::FileHelper>::allocation::dealloc(::org::nativescript::canvas::FileHelper *ptr) noexcept {
  cxxbridge1$box$org$nativescript$canvas$FileHelper$dealloc(ptr);
}
template <>
void Box<::org::nativescript::canvas::FileHelper>::drop() noexcept {
  cxxbridge1$box$org$nativescript$canvas$FileHelper$drop(this);
}
template <>
::org::nativescript::canvas::Raf *Box<::org::nativescript::canvas::Raf>::allocation::alloc() noexcept {
  return cxxbridge1$box$org$nativescript$canvas$Raf$alloc();
}
template <>
void Box<::org::nativescript::canvas::Raf>::allocation::dealloc(::org::nativescript::canvas::Raf *ptr) noexcept {
  cxxbridge1$box$org$nativescript$canvas$Raf$dealloc(ptr);
}
template <>
void Box<::org::nativescript::canvas::Raf>::drop() noexcept {
  cxxbridge1$box$org$nativescript$canvas$Raf$drop(this);
}
template <>
::org::nativescript::canvas::ImageAsset *Box<::org::nativescript::canvas::ImageAsset>::allocation::alloc() noexcept {
  return cxxbridge1$box$org$nativescript$canvas$ImageAsset$alloc();
}
template <>
void Box<::org::nativescript::canvas::ImageAsset>::allocation::dealloc(::org::nativescript::canvas::ImageAsset *ptr) noexcept {
  cxxbridge1$box$org$nativescript$canvas$ImageAsset$dealloc(ptr);
}
template <>
void Box<::org::nativescript::canvas::ImageAsset>::drop() noexcept {
  cxxbridge1$box$org$nativescript$canvas$ImageAsset$drop(this);
}
template <>
::org::nativescript::canvas::Path *Box<::org::nativescript::canvas::Path>::allocation::alloc() noexcept {
  return cxxbridge1$box$org$nativescript$canvas$Path$alloc();
}
template <>
void Box<::org::nativescript::canvas::Path>::allocation::dealloc(::org::nativescript::canvas::Path *ptr) noexcept {
  cxxbridge1$box$org$nativescript$canvas$Path$dealloc(ptr);
}
template <>
void Box<::org::nativescript::canvas::Path>::drop() noexcept {
  cxxbridge1$box$org$nativescript$canvas$Path$drop(this);
}
template <>
::org::nativescript::canvas::Matrix *Box<::org::nativescript::canvas::Matrix>::allocation::alloc() noexcept {
  return cxxbridge1$box$org$nativescript$canvas$Matrix$alloc();
}
template <>
void Box<::org::nativescript::canvas::Matrix>::allocation::dealloc(::org::nativescript::canvas::Matrix *ptr) noexcept {
  cxxbridge1$box$org$nativescript$canvas$Matrix$dealloc(ptr);
}
template <>
void Box<::org::nativescript::canvas::Matrix>::drop() noexcept {
  cxxbridge1$box$org$nativescript$canvas$Matrix$drop(this);
}
template <>
::org::nativescript::canvas::ImageData *Box<::org::nativescript::canvas::ImageData>::allocation::alloc() noexcept {
  return cxxbridge1$box$org$nativescript$canvas$ImageData$alloc();
}
template <>
void Box<::org::nativescript::canvas::ImageData>::allocation::dealloc(::org::nativescript::canvas::ImageData *ptr) noexcept {
  cxxbridge1$box$org$nativescript$canvas$ImageData$dealloc(ptr);
}
template <>
void Box<::org::nativescript::canvas::ImageData>::drop() noexcept {
  cxxbridge1$box$org$nativescript$canvas$ImageData$drop(this);
}
template <>
::org::nativescript::canvas::PaintStyle *Box<::org::nativescript::canvas::PaintStyle>::allocation::alloc() noexcept {
  return cxxbridge1$box$org$nativescript$canvas$PaintStyle$alloc();
}
template <>
void Box<::org::nativescript::canvas::PaintStyle>::allocation::dealloc(::org::nativescript::canvas::PaintStyle *ptr) noexcept {
  cxxbridge1$box$org$nativescript$canvas$PaintStyle$dealloc(ptr);
}
template <>
void Box<::org::nativescript::canvas::PaintStyle>::drop() noexcept {
  cxxbridge1$box$org$nativescript$canvas$PaintStyle$drop(this);
}
template <>
::org::nativescript::canvas::TextDecoder *Box<::org::nativescript::canvas::TextDecoder>::allocation::alloc() noexcept {
  return cxxbridge1$box$org$nativescript$canvas$TextDecoder$alloc();
}
template <>
void Box<::org::nativescript::canvas::TextDecoder>::allocation::dealloc(::org::nativescript::canvas::TextDecoder *ptr) noexcept {
  cxxbridge1$box$org$nativescript$canvas$TextDecoder$dealloc(ptr);
}
template <>
void Box<::org::nativescript::canvas::TextDecoder>::drop() noexcept {
  cxxbridge1$box$org$nativescript$canvas$TextDecoder$drop(this);
}
template <>
::org::nativescript::canvas::TextEncoder *Box<::org::nativescript::canvas::TextEncoder>::allocation::alloc() noexcept {
  return cxxbridge1$box$org$nativescript$canvas$TextEncoder$alloc();
}
template <>
void Box<::org::nativescript::canvas::TextEncoder>::allocation::dealloc(::org::nativescript::canvas::TextEncoder *ptr) noexcept {
  cxxbridge1$box$org$nativescript$canvas$TextEncoder$dealloc(ptr);
}
template <>
void Box<::org::nativescript::canvas::TextEncoder>::drop() noexcept {
  cxxbridge1$box$org$nativescript$canvas$TextEncoder$drop(this);
}
template <>
::org::nativescript::canvas::CanvasRenderingContext2D *Box<::org::nativescript::canvas::CanvasRenderingContext2D>::allocation::alloc() noexcept {
  return cxxbridge1$box$org$nativescript$canvas$CanvasRenderingContext2D$alloc();
}
template <>
void Box<::org::nativescript::canvas::CanvasRenderingContext2D>::allocation::dealloc(::org::nativescript::canvas::CanvasRenderingContext2D *ptr) noexcept {
  cxxbridge1$box$org$nativescript$canvas$CanvasRenderingContext2D$dealloc(ptr);
}
template <>
void Box<::org::nativescript::canvas::CanvasRenderingContext2D>::drop() noexcept {
  cxxbridge1$box$org$nativescript$canvas$CanvasRenderingContext2D$drop(this);
}
template <>
::org::nativescript::canvas::TextMetrics *Box<::org::nativescript::canvas::TextMetrics>::allocation::alloc() noexcept {
  return cxxbridge1$box$org$nativescript$canvas$TextMetrics$alloc();
}
template <>
void Box<::org::nativescript::canvas::TextMetrics>::allocation::dealloc(::org::nativescript::canvas::TextMetrics *ptr) noexcept {
  cxxbridge1$box$org$nativescript$canvas$TextMetrics$dealloc(ptr);
}
template <>
void Box<::org::nativescript::canvas::TextMetrics>::drop() noexcept {
  cxxbridge1$box$org$nativescript$canvas$TextMetrics$drop(this);
}
template <>
::org::nativescript::canvas::WebGLExtension *Box<::org::nativescript::canvas::WebGLExtension>::allocation::alloc() noexcept {
  return cxxbridge1$box$org$nativescript$canvas$WebGLExtension$alloc();
}
template <>
void Box<::org::nativescript::canvas::WebGLExtension>::allocation::dealloc(::org::nativescript::canvas::WebGLExtension *ptr) noexcept {
  cxxbridge1$box$org$nativescript$canvas$WebGLExtension$dealloc(ptr);
}
template <>
void Box<::org::nativescript::canvas::WebGLExtension>::drop() noexcept {
  cxxbridge1$box$org$nativescript$canvas$WebGLExtension$drop(this);
}
template <>
::org::nativescript::canvas::EXT_disjoint_timer_query *Box<::org::nativescript::canvas::EXT_disjoint_timer_query>::allocation::alloc() noexcept {
  return cxxbridge1$box$org$nativescript$canvas$EXT_disjoint_timer_query$alloc();
}
template <>
void Box<::org::nativescript::canvas::EXT_disjoint_timer_query>::allocation::dealloc(::org::nativescript::canvas::EXT_disjoint_timer_query *ptr) noexcept {
  cxxbridge1$box$org$nativescript$canvas$EXT_disjoint_timer_query$dealloc(ptr);
}
template <>
void Box<::org::nativescript::canvas::EXT_disjoint_timer_query>::drop() noexcept {
  cxxbridge1$box$org$nativescript$canvas$EXT_disjoint_timer_query$drop(this);
}
template <>
::org::nativescript::canvas::ANGLE_instanced_arrays *Box<::org::nativescript::canvas::ANGLE_instanced_arrays>::allocation::alloc() noexcept {
  return cxxbridge1$box$org$nativescript$canvas$ANGLE_instanced_arrays$alloc();
}
template <>
void Box<::org::nativescript::canvas::ANGLE_instanced_arrays>::allocation::dealloc(::org::nativescript::canvas::ANGLE_instanced_arrays *ptr) noexcept {
  cxxbridge1$box$org$nativescript$canvas$ANGLE_instanced_arrays$dealloc(ptr);
}
template <>
void Box<::org::nativescript::canvas::ANGLE_instanced_arrays>::drop() noexcept {
  cxxbridge1$box$org$nativescript$canvas$ANGLE_instanced_arrays$drop(this);
}
template <>
::org::nativescript::canvas::WEBGL_lose_context *Box<::org::nativescript::canvas::WEBGL_lose_context>::allocation::alloc() noexcept {
  return cxxbridge1$box$org$nativescript$canvas$WEBGL_lose_context$alloc();
}
template <>
void Box<::org::nativescript::canvas::WEBGL_lose_context>::allocation::dealloc(::org::nativescript::canvas::WEBGL_lose_context *ptr) noexcept {
  cxxbridge1$box$org$nativescript$canvas$WEBGL_lose_context$dealloc(ptr);
}
template <>
void Box<::org::nativescript::canvas::WEBGL_lose_context>::drop() noexcept {
  cxxbridge1$box$org$nativescript$canvas$WEBGL_lose_context$drop(this);
}
template <>
::org::nativescript::canvas::WEBGL_draw_buffers *Box<::org::nativescript::canvas::WEBGL_draw_buffers>::allocation::alloc() noexcept {
  return cxxbridge1$box$org$nativescript$canvas$WEBGL_draw_buffers$alloc();
}
template <>
void Box<::org::nativescript::canvas::WEBGL_draw_buffers>::allocation::dealloc(::org::nativescript::canvas::WEBGL_draw_buffers *ptr) noexcept {
  cxxbridge1$box$org$nativescript$canvas$WEBGL_draw_buffers$dealloc(ptr);
}
template <>
void Box<::org::nativescript::canvas::WEBGL_draw_buffers>::drop() noexcept {
  cxxbridge1$box$org$nativescript$canvas$WEBGL_draw_buffers$drop(this);
}
template <>
::org::nativescript::canvas::OES_vertex_array_object *Box<::org::nativescript::canvas::OES_vertex_array_object>::allocation::alloc() noexcept {
  return cxxbridge1$box$org$nativescript$canvas$OES_vertex_array_object$alloc();
}
template <>
void Box<::org::nativescript::canvas::OES_vertex_array_object>::allocation::dealloc(::org::nativescript::canvas::OES_vertex_array_object *ptr) noexcept {
  cxxbridge1$box$org$nativescript$canvas$OES_vertex_array_object$dealloc(ptr);
}
template <>
void Box<::org::nativescript::canvas::OES_vertex_array_object>::drop() noexcept {
  cxxbridge1$box$org$nativescript$canvas$OES_vertex_array_object$drop(this);
}
template <>
::org::nativescript::canvas::WebGLResult *Box<::org::nativescript::canvas::WebGLResult>::allocation::alloc() noexcept {
  return cxxbridge1$box$org$nativescript$canvas$WebGLResult$alloc();
}
template <>
void Box<::org::nativescript::canvas::WebGLResult>::allocation::dealloc(::org::nativescript::canvas::WebGLResult *ptr) noexcept {
  cxxbridge1$box$org$nativescript$canvas$WebGLResult$dealloc(ptr);
}
template <>
void Box<::org::nativescript::canvas::WebGLResult>::drop() noexcept {
  cxxbridge1$box$org$nativescript$canvas$WebGLResult$drop(this);
}
template <>
::org::nativescript::canvas::WebGLState *Box<::org::nativescript::canvas::WebGLState>::allocation::alloc() noexcept {
  return cxxbridge1$box$org$nativescript$canvas$WebGLState$alloc();
}
template <>
void Box<::org::nativescript::canvas::WebGLState>::allocation::dealloc(::org::nativescript::canvas::WebGLState *ptr) noexcept {
  cxxbridge1$box$org$nativescript$canvas$WebGLState$dealloc(ptr);
}
template <>
void Box<::org::nativescript::canvas::WebGLState>::drop() noexcept {
  cxxbridge1$box$org$nativescript$canvas$WebGLState$drop(this);
}
template <>
::org::nativescript::canvas::WebGLActiveInfo *Box<::org::nativescript::canvas::WebGLActiveInfo>::allocation::alloc() noexcept {
  return cxxbridge1$box$org$nativescript$canvas$WebGLActiveInfo$alloc();
}
template <>
void Box<::org::nativescript::canvas::WebGLActiveInfo>::allocation::dealloc(::org::nativescript::canvas::WebGLActiveInfo *ptr) noexcept {
  cxxbridge1$box$org$nativescript$canvas$WebGLActiveInfo$dealloc(ptr);
}
template <>
void Box<::org::nativescript::canvas::WebGLActiveInfo>::drop() noexcept {
  cxxbridge1$box$org$nativescript$canvas$WebGLActiveInfo$drop(this);
}
template <>
::org::nativescript::canvas::ContextAttributes *Box<::org::nativescript::canvas::ContextAttributes>::allocation::alloc() noexcept {
  return cxxbridge1$box$org$nativescript$canvas$ContextAttributes$alloc();
}
template <>
void Box<::org::nativescript::canvas::ContextAttributes>::allocation::dealloc(::org::nativescript::canvas::ContextAttributes *ptr) noexcept {
  cxxbridge1$box$org$nativescript$canvas$ContextAttributes$dealloc(ptr);
}
template <>
void Box<::org::nativescript::canvas::ContextAttributes>::drop() noexcept {
  cxxbridge1$box$org$nativescript$canvas$ContextAttributes$drop(this);
}
template <>
::org::nativescript::canvas::WebGLFramebufferAttachmentParameter *Box<::org::nativescript::canvas::WebGLFramebufferAttachmentParameter>::allocation::alloc() noexcept {
  return cxxbridge1$box$org$nativescript$canvas$WebGLFramebufferAttachmentParameter$alloc();
}
template <>
void Box<::org::nativescript::canvas::WebGLFramebufferAttachmentParameter>::allocation::dealloc(::org::nativescript::canvas::WebGLFramebufferAttachmentParameter *ptr) noexcept {
  cxxbridge1$box$org$nativescript$canvas$WebGLFramebufferAttachmentParameter$dealloc(ptr);
}
template <>
void Box<::org::nativescript::canvas::WebGLFramebufferAttachmentParameter>::drop() noexcept {
  cxxbridge1$box$org$nativescript$canvas$WebGLFramebufferAttachmentParameter$drop(this);
}
template <>
::org::nativescript::canvas::WebGLShaderPrecisionFormat *Box<::org::nativescript::canvas::WebGLShaderPrecisionFormat>::allocation::alloc() noexcept {
  return cxxbridge1$box$org$nativescript$canvas$WebGLShaderPrecisionFormat$alloc();
}
template <>
void Box<::org::nativescript::canvas::WebGLShaderPrecisionFormat>::allocation::dealloc(::org::nativescript::canvas::WebGLShaderPrecisionFormat *ptr) noexcept {
  cxxbridge1$box$org$nativescript$canvas$WebGLShaderPrecisionFormat$dealloc(ptr);
}
template <>
void Box<::org::nativescript::canvas::WebGLShaderPrecisionFormat>::drop() noexcept {
  cxxbridge1$box$org$nativescript$canvas$WebGLShaderPrecisionFormat$drop(this);
}
template <>
::org::nativescript::canvas::WebGLSync *Box<::org::nativescript::canvas::WebGLSync>::allocation::alloc() noexcept {
  return cxxbridge1$box$org$nativescript$canvas$WebGLSync$alloc();
}
template <>
void Box<::org::nativescript::canvas::WebGLSync>::allocation::dealloc(::org::nativescript::canvas::WebGLSync *ptr) noexcept {
  cxxbridge1$box$org$nativescript$canvas$WebGLSync$dealloc(ptr);
}
template <>
void Box<::org::nativescript::canvas::WebGLSync>::drop() noexcept {
  cxxbridge1$box$org$nativescript$canvas$WebGLSync$drop(this);
}
template <>
::org::nativescript::canvas::WebGLIndexedParameter *Box<::org::nativescript::canvas::WebGLIndexedParameter>::allocation::alloc() noexcept {
  return cxxbridge1$box$org$nativescript$canvas$WebGLIndexedParameter$alloc();
}
template <>
void Box<::org::nativescript::canvas::WebGLIndexedParameter>::allocation::dealloc(::org::nativescript::canvas::WebGLIndexedParameter *ptr) noexcept {
  cxxbridge1$box$org$nativescript$canvas$WebGLIndexedParameter$dealloc(ptr);
}
template <>
void Box<::org::nativescript::canvas::WebGLIndexedParameter>::drop() noexcept {
  cxxbridge1$box$org$nativescript$canvas$WebGLIndexedParameter$drop(this);
}
} // namespace cxxbridge1
} // namespace rust
