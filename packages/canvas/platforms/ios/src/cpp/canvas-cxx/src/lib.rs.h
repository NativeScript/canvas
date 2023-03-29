#pragma once
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
  None = 24,
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

::rust::Box<::org::nativescript::canvas::FileHelper> canvas_native_helper_read_file(::rust::Str path) noexcept;

bool canvas_native_helper_read_file_has_error(::org::nativescript::canvas::FileHelper const &file) noexcept;

::rust::Vec<::std::uint8_t> canvas_native_helper_read_file_get_data(::rust::Box<::org::nativescript::canvas::FileHelper> file) noexcept;

::rust::String canvas_native_helper_read_file_get_error(::org::nativescript::canvas::FileHelper const &file) noexcept;

::rust::Box<::org::nativescript::canvas::Raf> canvas_native_raf_create(::rust::isize callback) noexcept;

void canvas_native_raf_start(::org::nativescript::canvas::Raf &raf) noexcept;

void canvas_native_raf_stop(::org::nativescript::canvas::Raf &raf) noexcept;

bool canvas_native_raf_get_started(::org::nativescript::canvas::Raf const &raf) noexcept;

void console_log(::rust::Str text) noexcept;

::rust::Vec<::std::uint8_t> str_to_buf(::rust::Str value) noexcept;

::rust::Box<::org::nativescript::canvas::ImageAsset> canvas_native_image_bitmap_create_from_asset(::org::nativescript::canvas::ImageAsset &asset, bool flip_y, ::org::nativescript::canvas::ImageBitmapPremultiplyAlpha premultiply_alpha, ::org::nativescript::canvas::ImageBitmapColorSpaceConversion color_space_conversion, ::org::nativescript::canvas::ImageBitmapResizeQuality resize_quality, float resize_width, float resize_height) noexcept;

::rust::Box<::org::nativescript::canvas::ImageAsset> canvas_native_image_bitmap_create_from_asset_src_rect(::org::nativescript::canvas::ImageAsset &asset, float sx, float sy, float s_width, float s_height, bool flip_y, ::org::nativescript::canvas::ImageBitmapPremultiplyAlpha premultiply_alpha, ::org::nativescript::canvas::ImageBitmapColorSpaceConversion color_space_conversion, ::org::nativescript::canvas::ImageBitmapResizeQuality resize_quality, float resize_width, float resize_height) noexcept;

::rust::Box<::org::nativescript::canvas::ImageAsset> canvas_native_image_bitmap_create_from_encoded_bytes(::rust::Slice<::std::uint8_t const> bytes, bool flip_y, ::org::nativescript::canvas::ImageBitmapPremultiplyAlpha premultiply_alpha, ::org::nativescript::canvas::ImageBitmapColorSpaceConversion color_space_conversion, ::org::nativescript::canvas::ImageBitmapResizeQuality resize_quality, float resize_width, float resize_height) noexcept;

bool canvas_native_image_bitmap_create_from_encoded_bytes_with_output(::rust::Slice<::std::uint8_t const> bytes, bool flip_y, ::org::nativescript::canvas::ImageBitmapPremultiplyAlpha premultiply_alpha, ::org::nativescript::canvas::ImageBitmapColorSpaceConversion color_space_conversion, ::org::nativescript::canvas::ImageBitmapResizeQuality resize_quality, float resize_width, float resize_height, ::org::nativescript::canvas::ImageAsset &output) noexcept;

::rust::Box<::org::nativescript::canvas::ImageAsset> canvas_native_image_bitmap_create_from_encoded_bytes_src_rect(::rust::Slice<::std::uint8_t const> bytes, float sx, float sy, float s_width, float s_height, bool flip_y, ::org::nativescript::canvas::ImageBitmapPremultiplyAlpha premultiply_alpha, ::org::nativescript::canvas::ImageBitmapColorSpaceConversion color_space_conversion, ::org::nativescript::canvas::ImageBitmapResizeQuality resize_quality, float resize_width, float resize_height) noexcept;

bool canvas_native_image_bitmap_create_from_encoded_bytes_src_rect_with_output(::rust::Slice<::std::uint8_t const> bytes, float sx, float sy, float s_width, float s_height, bool flip_y, ::org::nativescript::canvas::ImageBitmapPremultiplyAlpha premultiply_alpha, ::org::nativescript::canvas::ImageBitmapColorSpaceConversion color_space_conversion, ::org::nativescript::canvas::ImageBitmapResizeQuality resize_quality, float resize_width, float resize_height, ::org::nativescript::canvas::ImageAsset &output) noexcept;

void canvas_native_path_add_path(::org::nativescript::canvas::Path &path, ::org::nativescript::canvas::Path const &path_to_add) noexcept;

::rust::Box<::org::nativescript::canvas::Path> canvas_native_path_create() noexcept;

::rust::Box<::org::nativescript::canvas::Path> canvas_native_path_create_with_path(::org::nativescript::canvas::Path const &path) noexcept;

::rust::Box<::org::nativescript::canvas::Path> canvas_native_path_create_with_string(::rust::String string) noexcept;

::rust::Box<::org::nativescript::canvas::Path> canvas_native_path_create_with_str(::rust::Str string) noexcept;

void canvas_native_path_close_path(::org::nativescript::canvas::Path &path) noexcept;

void canvas_native_path_move_to(::org::nativescript::canvas::Path &path, float x, float y) noexcept;

void canvas_native_path_line_to(::org::nativescript::canvas::Path &path, float x, float y) noexcept;

void canvas_native_path_bezier_curve_to(::org::nativescript::canvas::Path &path, float cp1x, float cp1y, float cp2x, float cp2y, float x, float y) noexcept;

void canvas_native_path_quadratic_curve_to(::org::nativescript::canvas::Path &path, float cpx, float cpy, float x, float y) noexcept;

void canvas_native_path_arc(::org::nativescript::canvas::Path &path, float x, float y, float radius, float start_angle, float end_angle, bool anti_clockwise) noexcept;

void canvas_native_path_arc_to(::org::nativescript::canvas::Path &path, float x1, float y1, float x2, float y2, float radius) noexcept;

void canvas_native_path_ellipse(::org::nativescript::canvas::Path &path, float x, float y, float radius_x, float radius_y, float rotation, float start_angle, float end_angle, bool anticlockwise) noexcept;

void canvas_native_path_rect(::org::nativescript::canvas::Path &path, float x, float y, float width, float height) noexcept;

void canvas_native_path_round_rect(::org::nativescript::canvas::Path &path, float x, float y, float width, float height, ::rust::Slice<float const> radii) noexcept;

void canvas_native_path_round_rect_tl_tr_br_bl(::org::nativescript::canvas::Path &path, float x, float y, float width, float height, float top_left, float top_right, float bottom_right, float bottom_left) noexcept;

::rust::String canvas_native_path_to_string(::org::nativescript::canvas::Path const &path) noexcept;

::rust::Box<::org::nativescript::canvas::Matrix> canvas_native_matrix_create() noexcept;

void canvas_native_matrix_update(::org::nativescript::canvas::Matrix &matrix, ::rust::Slice<float const> slice) noexcept;

void canvas_native_matrix_update_3d(::org::nativescript::canvas::Matrix &matrix, ::std::array<float, 16> const &slice) noexcept;

float canvas_native_matrix_get_a(::org::nativescript::canvas::Matrix const &matrix) noexcept;

void canvas_native_matrix_set_a(::org::nativescript::canvas::Matrix &matrix, float a) noexcept;

float canvas_native_matrix_get_b(::org::nativescript::canvas::Matrix const &matrix) noexcept;

void canvas_native_matrix_set_b(::org::nativescript::canvas::Matrix &matrix, float b) noexcept;

float canvas_native_matrix_get_c(::org::nativescript::canvas::Matrix const &matrix) noexcept;

void canvas_native_matrix_set_c(::org::nativescript::canvas::Matrix &matrix, float c) noexcept;

float canvas_native_matrix_get_d(::org::nativescript::canvas::Matrix const &matrix) noexcept;

void canvas_native_matrix_set_d(::org::nativescript::canvas::Matrix &matrix, float d) noexcept;

float canvas_native_matrix_get_e(::org::nativescript::canvas::Matrix const &matrix) noexcept;

void canvas_native_matrix_set_e(::org::nativescript::canvas::Matrix &matrix, float e) noexcept;

float canvas_native_matrix_get_f(::org::nativescript::canvas::Matrix const &matrix) noexcept;

void canvas_native_matrix_set_f(::org::nativescript::canvas::Matrix &matrix, float f) noexcept;

float canvas_native_matrix_get_m11(::org::nativescript::canvas::Matrix const &matrix) noexcept;

void canvas_native_matrix_set_m11(::org::nativescript::canvas::Matrix &matrix, float m11) noexcept;

float canvas_native_matrix_get_m12(::org::nativescript::canvas::Matrix const &matrix) noexcept;

void canvas_native_matrix_set_m12(::org::nativescript::canvas::Matrix &matrix, float m12) noexcept;

float canvas_native_matrix_get_m13(::org::nativescript::canvas::Matrix const &matrix) noexcept;

void canvas_native_matrix_set_m13(::org::nativescript::canvas::Matrix &matrix, float m13) noexcept;

float canvas_native_matrix_get_m14(::org::nativescript::canvas::Matrix const &matrix) noexcept;

void canvas_native_matrix_set_m14(::org::nativescript::canvas::Matrix &matrix, float m14) noexcept;

float canvas_native_matrix_get_m21(::org::nativescript::canvas::Matrix const &matrix) noexcept;

void canvas_native_matrix_set_m21(::org::nativescript::canvas::Matrix &matrix, float m21) noexcept;

float canvas_native_matrix_get_m22(::org::nativescript::canvas::Matrix const &matrix) noexcept;

void canvas_native_matrix_set_m22(::org::nativescript::canvas::Matrix &matrix, float m22) noexcept;

float canvas_native_matrix_get_m23(::org::nativescript::canvas::Matrix const &matrix) noexcept;

void canvas_native_matrix_set_m23(::org::nativescript::canvas::Matrix &matrix, float m23) noexcept;

float canvas_native_matrix_get_m24(::org::nativescript::canvas::Matrix const &matrix) noexcept;

void canvas_native_matrix_set_m24(::org::nativescript::canvas::Matrix &matrix, float m24) noexcept;

float canvas_native_matrix_get_m31(::org::nativescript::canvas::Matrix const &matrix) noexcept;

void canvas_native_matrix_set_m31(::org::nativescript::canvas::Matrix &matrix, float m31) noexcept;

float canvas_native_matrix_get_m32(::org::nativescript::canvas::Matrix const &matrix) noexcept;

void canvas_native_matrix_set_m32(::org::nativescript::canvas::Matrix &matrix, float m32) noexcept;

float canvas_native_matrix_get_m33(::org::nativescript::canvas::Matrix const &matrix) noexcept;

void canvas_native_matrix_set_m33(::org::nativescript::canvas::Matrix &matrix, float m33) noexcept;

float canvas_native_matrix_get_m34(::org::nativescript::canvas::Matrix const &matrix) noexcept;

void canvas_native_matrix_set_m34(::org::nativescript::canvas::Matrix &matrix, float m34) noexcept;

float canvas_native_matrix_get_m41(::org::nativescript::canvas::Matrix const &matrix) noexcept;

void canvas_native_matrix_set_m41(::org::nativescript::canvas::Matrix &matrix, float m41) noexcept;

float canvas_native_matrix_get_m42(::org::nativescript::canvas::Matrix const &matrix) noexcept;

void canvas_native_matrix_set_m42(::org::nativescript::canvas::Matrix &matrix, float m42) noexcept;

float canvas_native_matrix_get_m43(::org::nativescript::canvas::Matrix const &matrix) noexcept;

void canvas_native_matrix_set_m43(::org::nativescript::canvas::Matrix &matrix, float m43) noexcept;

float canvas_native_matrix_get_m44(::org::nativescript::canvas::Matrix const &matrix) noexcept;

void canvas_native_matrix_set_m44(::org::nativescript::canvas::Matrix &matrix, float m44) noexcept;

::rust::Box<::org::nativescript::canvas::ImageData> canvas_native_image_data_create(::std::int32_t width, ::std::int32_t height) noexcept;

::std::int32_t canvas_native_image_data_get_width(::org::nativescript::canvas::ImageData const &image_data) noexcept;

::std::int32_t canvas_native_image_data_get_height(::org::nativescript::canvas::ImageData const &image_data) noexcept;

::rust::Slice<::std::uint8_t > canvas_native_image_data_get_data(::org::nativescript::canvas::ImageData &image_data) noexcept;

::rust::Box<::org::nativescript::canvas::ImageData> canvas_native_image_data_get_shared_instance(::org::nativescript::canvas::ImageData &image_data) noexcept;

::rust::Box<::org::nativescript::canvas::ImageAsset> canvas_native_image_asset_create() noexcept;

::rust::Box<::org::nativescript::canvas::ImageAsset> canvas_native_image_asset_shared_clone(::org::nativescript::canvas::ImageAsset const &asset) noexcept;

bool canvas_native_image_asset_load_from_fd(::org::nativescript::canvas::ImageAsset &asset, ::std::int32_t fd) noexcept;

bool canvas_native_image_asset_load_from_path(::org::nativescript::canvas::ImageAsset &asset, ::rust::Str path) noexcept;

bool canvas_native_image_asset_load_from_url(::org::nativescript::canvas::ImageAsset &asset, ::rust::Str url) noexcept;

bool canvas_native_image_asset_load_from_raw(::org::nativescript::canvas::ImageAsset &asset, ::rust::Slice<::std::uint8_t const> array) noexcept;

::std::int64_t canvas_native_image_asset_addr(::org::nativescript::canvas::ImageAsset &asset) noexcept;

::std::uint32_t canvas_native_image_asset_width(::org::nativescript::canvas::ImageAsset &asset) noexcept;

::std::uint32_t canvas_native_image_asset_height(::org::nativescript::canvas::ImageAsset &asset) noexcept;

::rust::String canvas_native_image_asset_get_error(::org::nativescript::canvas::ImageAsset &asset) noexcept;

bool canvas_native_image_asset_has_error(::org::nativescript::canvas::ImageAsset &asset) noexcept;

bool canvas_native_image_asset_scale(::org::nativescript::canvas::ImageAsset &asset, ::std::uint32_t x, ::std::uint32_t y) noexcept;

bool canvas_native_image_asset_save_path(::org::nativescript::canvas::ImageAsset &asset, ::rust::Str path, ::std::uint32_t format) noexcept;

float canvas_native_text_metrics_get_width(::org::nativescript::canvas::TextMetrics const &metrics) noexcept;

float canvas_native_text_metrics_get_actual_bounding_box_left(::org::nativescript::canvas::TextMetrics const &metrics) noexcept;

float canvas_native_text_metrics_get_actual_bounding_box_right(::org::nativescript::canvas::TextMetrics const &metrics) noexcept;

float canvas_native_text_metrics_get_actual_bounding_box_ascent(::org::nativescript::canvas::TextMetrics const &metrics) noexcept;

float canvas_native_text_metrics_get_actual_bounding_box_descent(::org::nativescript::canvas::TextMetrics const &metrics) noexcept;

float canvas_native_text_metrics_get_font_bounding_box_ascent(::org::nativescript::canvas::TextMetrics const &metrics) noexcept;

float canvas_native_text_metrics_get_font_bounding_box_descent(::org::nativescript::canvas::TextMetrics const &metrics) noexcept;

float canvas_native_text_metrics_get_em_height_ascent(::org::nativescript::canvas::TextMetrics const &metrics) noexcept;

float canvas_native_text_metrics_get_em_height_descent(::org::nativescript::canvas::TextMetrics const &metrics) noexcept;

float canvas_native_text_metrics_get_hanging_baseline(::org::nativescript::canvas::TextMetrics const &metrics) noexcept;

float canvas_native_text_metrics_get_alphabetic_baseline(::org::nativescript::canvas::TextMetrics const &metrics) noexcept;

float canvas_native_text_metrics_get_ideographic_baseline(::org::nativescript::canvas::TextMetrics const &metrics) noexcept;

::rust::Box<::org::nativescript::canvas::PaintStyle> canvas_native_paint_style_from_bytes(::org::nativescript::canvas::CanvasRenderingContext2D const &context, ::std::int32_t repetition, ::std::int32_t width, ::std::int32_t height, ::rust::Slice<::std::uint8_t const> bytes) noexcept;

::rust::Box<::org::nativescript::canvas::PaintStyle> canvas_native_pattern_from_ptr(::std::int64_t ptr) noexcept;

::rust::Box<::org::nativescript::canvas::PaintStyle> canvas_native_paint_style_empty() noexcept;

void canvas_native_gradient_add_color_stop(::org::nativescript::canvas::PaintStyle &style, float stop, ::rust::Str color) noexcept;

void canvas_native_pattern_set_transform(::org::nativescript::canvas::PaintStyle &pattern, ::org::nativescript::canvas::Matrix const &matrix) noexcept;

::rust::Box<::org::nativescript::canvas::TextDecoder> canvas_native_text_decoder_create(::rust::Str decoding) noexcept;

::rust::String canvas_native_text_decoder_get_encoding(::org::nativescript::canvas::TextDecoder &decoder) noexcept;

::rust::String canvas_native_text_decoder_decode(::org::nativescript::canvas::TextDecoder &decoder, ::rust::Slice<::std::uint8_t const> data) noexcept;

::rust::Box<::org::nativescript::canvas::TextEncoder> canvas_native_text_encoder_create(::rust::Str encoding) noexcept;

::rust::String canvas_native_text_encoder_get_encoding(::org::nativescript::canvas::TextEncoder &decoder) noexcept;

::rust::Vec<::std::uint8_t> canvas_native_text_encoder_encode(::org::nativescript::canvas::TextEncoder &encoder, ::rust::Str text) noexcept;

bool canvas_native_context_gl_make_current(::org::nativescript::canvas::CanvasRenderingContext2D const &context) noexcept;

bool canvas_native_context_gl_swap_buffers(::org::nativescript::canvas::CanvasRenderingContext2D const &context) noexcept;

::rust::Box<::org::nativescript::canvas::CanvasRenderingContext2D> canvas_native_context_create_with_wrapper(::std::int64_t context, ::std::int64_t gl_context) noexcept;

void canvas_native_context_resize(::org::nativescript::canvas::CanvasRenderingContext2D &context, float width, float height) noexcept;

::rust::Box<::org::nativescript::canvas::CanvasRenderingContext2D> canvas_native_context_create(float width, float height, float density, bool alpha, ::std::int32_t font_color, float ppi, ::std::uint32_t direction) noexcept;

::rust::Box<::org::nativescript::canvas::CanvasRenderingContext2D> canvas_native_context_create_gl(float width, float height, float density, ::std::int64_t gl_context, ::std::int32_t samples, bool alpha, ::std::int32_t font_color, float ppi, ::std::uint32_t direction) noexcept;

::rust::Box<::org::nativescript::canvas::CanvasRenderingContext2D> canvas_native_context_create_with_pointer(::std::int64_t pointer) noexcept;

::rust::Box<::org::nativescript::canvas::CanvasRenderingContext2D> canvas_native_context_create_gl_no_window(float width, float height, float density, ::std::int32_t font_color, float ppi, ::std::uint32_t direction, bool alpha) noexcept;

void canvas_native_context_flush(::org::nativescript::canvas::CanvasRenderingContext2D const &context) noexcept;

void canvas_native_context_render(::org::nativescript::canvas::CanvasRenderingContext2D const &context) noexcept;

::rust::String canvas_native_to_data_url(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::rust::Str format, ::std::int32_t quality) noexcept;

::rust::String canvas_native_to_data_url_string(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::rust::String format, ::std::int32_t quality) noexcept;

::rust::String canvas_native_to_data_url_c_string(::org::nativescript::canvas::CanvasRenderingContext2D &context, char const *format, ::std::int32_t quality) noexcept;

::rust::String canvas_native_context_get_filter(::org::nativescript::canvas::CanvasRenderingContext2D const &context) noexcept;

void canvas_native_context_set_filter(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::rust::Str font) noexcept;

::rust::String canvas_native_context_get_font(::org::nativescript::canvas::CanvasRenderingContext2D const &context) noexcept;

void canvas_native_context_set_font(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::rust::Str font) noexcept;

float canvas_native_context_get_global_alpha(::org::nativescript::canvas::CanvasRenderingContext2D const &context) noexcept;

void canvas_native_context_set_global_alpha(::org::nativescript::canvas::CanvasRenderingContext2D &context, float alpha) noexcept;

bool canvas_native_context_get_image_smoothing_enabled(::org::nativescript::canvas::CanvasRenderingContext2D const &context) noexcept;

void canvas_native_context_set_image_smoothing_enabled(::org::nativescript::canvas::CanvasRenderingContext2D &context, bool enabled) noexcept;

::rust::Str canvas_native_context_get_image_smoothing_quality(::org::nativescript::canvas::CanvasRenderingContext2D const &context) noexcept;

void canvas_native_context_set_image_smoothing_quality(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::rust::Str quality) noexcept;

::rust::Str canvas_native_context_get_line_join(::org::nativescript::canvas::CanvasRenderingContext2D const &context) noexcept;

void canvas_native_context_set_line_join(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::rust::Str join) noexcept;

::rust::Str canvas_native_context_get_line_cap(::org::nativescript::canvas::CanvasRenderingContext2D const &context) noexcept;

void canvas_native_context_set_line_cap(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::rust::Str cap) noexcept;

float canvas_native_context_get_miter_limit(::org::nativescript::canvas::CanvasRenderingContext2D const &context) noexcept;

void canvas_native_context_set_miter_limit(::org::nativescript::canvas::CanvasRenderingContext2D &context, float limit) noexcept;

::rust::String canvas_native_context_get_shadow_color(::org::nativescript::canvas::CanvasRenderingContext2D const &context) noexcept;

::rust::Vec<::std::uint8_t> canvas_native_context_get_shadow_color_buf(::org::nativescript::canvas::CanvasRenderingContext2D const &context) noexcept;

void canvas_native_context_get_shadow_color_rgba(::org::nativescript::canvas::CanvasRenderingContext2D const &context, ::std::uint8_t &r, ::std::uint8_t &g, ::std::uint8_t &b, ::std::uint8_t &a) noexcept;

void canvas_native_context_set_shadow_color(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::rust::Str color) noexcept;

void canvas_native_context_set_shadow_color_rgba(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::std::uint8_t r, ::std::uint8_t g, ::std::uint8_t b, ::std::uint8_t a) noexcept;

float canvas_native_context_get_shadow_blur(::org::nativescript::canvas::CanvasRenderingContext2D const &context) noexcept;

void canvas_native_context_set_shadow_blur(::org::nativescript::canvas::CanvasRenderingContext2D &context, float blur) noexcept;

float canvas_native_context_get_shadow_offset_x(::org::nativescript::canvas::CanvasRenderingContext2D const &context) noexcept;

void canvas_native_context_set_shadow_offset_x(::org::nativescript::canvas::CanvasRenderingContext2D &context, float x) noexcept;

float canvas_native_context_get_shadow_offset_y(::org::nativescript::canvas::CanvasRenderingContext2D const &context) noexcept;

void canvas_native_context_set_shadow_offset_y(::org::nativescript::canvas::CanvasRenderingContext2D &context, float y) noexcept;

::rust::Str canvas_native_context_get_text_align(::org::nativescript::canvas::CanvasRenderingContext2D const &context) noexcept;

void canvas_native_context_set_text_align(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::rust::Str alignment) noexcept;

::rust::Str canvas_native_context_get_global_composition(::org::nativescript::canvas::CanvasRenderingContext2D const &context) noexcept;

void canvas_native_context_set_global_composition(::org::nativescript::canvas::CanvasRenderingContext2D const &context, ::rust::Str composition) noexcept;

void canvas_native_paint_style_set_fill_color_with_string(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::rust::Str color) noexcept;

bool canvas_native_parse_css_color_rgba(::rust::Str value, ::std::uint8_t &r, ::std::uint8_t &g, ::std::uint8_t &b, ::std::uint8_t &a) noexcept;

void canvas_native_paint_style_set_stroke_color_with_rgba(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::std::uint8_t r, ::std::uint8_t g, ::std::uint8_t b, ::std::uint8_t a) noexcept;

void canvas_native_paint_style_set_fill_color_with_rgba(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::std::uint8_t r, ::std::uint8_t g, ::std::uint8_t b, ::std::uint8_t a) noexcept;

void canvas_native_paint_style_set_stroke_color_with_string(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::rust::Str color) noexcept;

::rust::String canvas_native_paint_style_get_color_string(::org::nativescript::canvas::PaintStyle &color) noexcept;

::rust::String canvas_native_paint_style_get_current_stroke_color_string(::org::nativescript::canvas::CanvasRenderingContext2D &context) noexcept;

::rust::Vec<::std::uint8_t> canvas_native_paint_style_get_current_stroke_color_buf(::org::nativescript::canvas::CanvasRenderingContext2D &context) noexcept;

void canvas_native_paint_style_get_current_stroke_color_r_g_b_a(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::std::uint8_t &r, ::std::uint8_t &g, ::std::uint8_t &b, ::std::uint8_t &a) noexcept;

::rust::String canvas_native_paint_style_get_current_fill_color_string(::org::nativescript::canvas::CanvasRenderingContext2D &context) noexcept;

::rust::Vec<::std::uint8_t> canvas_native_paint_style_get_current_fill_color_buf(::org::nativescript::canvas::CanvasRenderingContext2D &context) noexcept;

void canvas_native_paint_style_get_current_fill_color_r_g_b_a(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::std::uint8_t &r, ::std::uint8_t &g, ::std::uint8_t &b, ::std::uint8_t &a) noexcept;

::org::nativescript::canvas::PaintStyleType canvas_native_context_get_style_type(::org::nativescript::canvas::PaintStyle const &style) noexcept;

::org::nativescript::canvas::PaintStyleType canvas_native_context_get_current_fill_style_type(::org::nativescript::canvas::CanvasRenderingContext2D &context) noexcept;

::org::nativescript::canvas::PaintStyleType canvas_native_context_get_current_stroke_style_type(::org::nativescript::canvas::CanvasRenderingContext2D &context) noexcept;

::rust::Box<::org::nativescript::canvas::PaintStyle> canvas_native_context_get_fill_style(::org::nativescript::canvas::CanvasRenderingContext2D &context) noexcept;

void canvas_native_context_set_fill_style(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::org::nativescript::canvas::PaintStyle const &style) noexcept;

::rust::Box<::org::nativescript::canvas::PaintStyle> canvas_native_context_get_stroke_style(::org::nativescript::canvas::CanvasRenderingContext2D const &context) noexcept;

void canvas_native_context_set_stroke_style(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::org::nativescript::canvas::PaintStyle const &style) noexcept;

float canvas_native_context_get_line_width(::org::nativescript::canvas::CanvasRenderingContext2D const &context) noexcept;

void canvas_native_context_set_line_width(::org::nativescript::canvas::CanvasRenderingContext2D &context, float width) noexcept;

float canvas_native_context_get_line_dash_offset(::org::nativescript::canvas::CanvasRenderingContext2D const &context) noexcept;

void canvas_native_context_set_line_dash_offset(::org::nativescript::canvas::CanvasRenderingContext2D &context, float offset) noexcept;

::rust::Vec<float> canvas_native_context_get_line_dash(::org::nativescript::canvas::CanvasRenderingContext2D const &context) noexcept;

void canvas_native_context_set_line_dash(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::rust::Slice<float const> dash) noexcept;

void canvas_native_context_arc(::org::nativescript::canvas::CanvasRenderingContext2D &context, float x, float y, float radius, float start_angle, float end_angle, bool anticlockwise) noexcept;

void canvas_native_context_arc_to(::org::nativescript::canvas::CanvasRenderingContext2D &context, float x1, float y1, float x2, float y2, float radius) noexcept;

void canvas_native_context_begin_path(::org::nativescript::canvas::CanvasRenderingContext2D &context) noexcept;

void canvas_native_context_bezier_curve_to(::org::nativescript::canvas::CanvasRenderingContext2D &context, float cp1x, float cp1y, float cp2x, float cp2y, float x, float y) noexcept;

void canvas_native_context_clear_rect(::org::nativescript::canvas::CanvasRenderingContext2D &context, float x, float y, float width, float height) noexcept;

void canvas_native_context_clip(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::org::nativescript::canvas::Path &path, ::rust::Str rule) noexcept;

void canvas_native_context_clip_rule(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::rust::Str rule) noexcept;

void canvas_native_context_close_path(::org::nativescript::canvas::CanvasRenderingContext2D &context) noexcept;

::rust::Box<::org::nativescript::canvas::ImageData> canvas_native_context_create_image_data(::std::int32_t width, ::std::int32_t height) noexcept;

::rust::Box<::org::nativescript::canvas::ImageData> canvas_native_context_create_image_data_with_data(::std::int32_t width, ::std::int32_t height, ::rust::Slice<::std::uint8_t const> data) noexcept;

::rust::Box<::org::nativescript::canvas::PaintStyle> canvas_native_context_create_linear_gradient(::org::nativescript::canvas::CanvasRenderingContext2D &context, float x0, float y0, float x1, float y1) noexcept;

::rust::Box<::org::nativescript::canvas::PaintStyle> canvas_native_context_create_pattern(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::rust::Slice<::std::uint8_t const> data, ::std::int32_t width, ::std::int32_t height, ::rust::Str repetition) noexcept;

::rust::Box<::org::nativescript::canvas::PaintStyle> canvas_native_context_create_pattern_asset(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::org::nativescript::canvas::ImageAsset &asset, ::rust::Str repetition) noexcept;

::rust::Box<::org::nativescript::canvas::PaintStyle> canvas_native_context_create_pattern_canvas2d(::org::nativescript::canvas::CanvasRenderingContext2D &source, ::org::nativescript::canvas::CanvasRenderingContext2D &context, ::rust::Str repetition) noexcept;

::rust::Box<::org::nativescript::canvas::PaintStyle> canvas_native_context_create_pattern_encoded(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::rust::Slice<::std::uint8_t const> data, ::rust::Str repetition) noexcept;

::rust::Box<::org::nativescript::canvas::PaintStyle> canvas_native_context_create_radial_gradient(::org::nativescript::canvas::CanvasRenderingContext2D &context, float x0, float y0, float r0, float x1, float y1, float r1) noexcept;

void canvas_native_context_draw_paint(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::rust::Str color) noexcept;

void canvas_native_context_draw_point(::org::nativescript::canvas::CanvasRenderingContext2D &context, float x, float y) noexcept;

void canvas_native_context_draw_points(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::std::int32_t mode, ::rust::Slice<float const> points) noexcept;

void canvas_native_context_draw_image_dx_dy(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::rust::Slice<::std::uint8_t const> data, float width, float height, float dx, float dy) noexcept;

void canvas_native_context_draw_image_dx_dy_dw_dh(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::rust::Slice<::std::uint8_t const> data, float width, float height, float dx, float dy, float d_width, float d_height) noexcept;

void canvas_native_context_draw_image(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::rust::Slice<::std::uint8_t const> data, float width, float height, float sx, float sy, float s_width, float s_height, float dx, float dy, float d_width, float d_height) noexcept;

void canvas_native_context_draw_image_encoded_dx_dy(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::rust::Slice<::std::uint8_t const> data, float dx, float dy) noexcept;

void canvas_native_context_draw_image_encoded_dx_dy_dw_dh(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::rust::Slice<::std::uint8_t const> data, float dx, float dy, float d_width, float d_height) noexcept;

void canvas_native_context_draw_image_encoded(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::rust::Slice<::std::uint8_t const> data, float sx, float sy, float s_width, float s_height, float dx, float dy, float d_width, float d_height) noexcept;

void canvas_native_context_draw_image_dx_dy_context(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::org::nativescript::canvas::CanvasRenderingContext2D &source, float dx, float dy) noexcept;

void canvas_native_context_draw_image_dx_dy_dw_dh_context(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::org::nativescript::canvas::CanvasRenderingContext2D &source, float dx, float dy, float d_width, float d_height) noexcept;

void canvas_native_context_draw_image_context(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::org::nativescript::canvas::CanvasRenderingContext2D &source, float sx, float sy, float s_width, float s_height, float dx, float dy, float d_width, float d_height) noexcept;

void canvas_native_context_draw_image_dx_dy_asset(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::org::nativescript::canvas::ImageAsset &asset, float dx, float dy) noexcept;

void canvas_native_context_draw_image_dx_dy_dw_dh_asset(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::org::nativescript::canvas::ImageAsset &asset, float dx, float dy, float d_width, float d_height) noexcept;

void canvas_native_context_draw_image_asset(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::org::nativescript::canvas::ImageAsset &asset, float sx, float sy, float s_width, float s_height, float dx, float dy, float d_width, float d_height) noexcept;

void canvas_native_context_ellipse(::org::nativescript::canvas::CanvasRenderingContext2D &context, float x, float y, float radius_x, float radius_y, float rotation, float start_angle, float end_angle, bool anticlockwise) noexcept;

void canvas_native_context_fill(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::rust::Str rule) noexcept;

void canvas_native_context_fill_with_path(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::org::nativescript::canvas::Path &path, ::rust::Str rule) noexcept;

void canvas_native_context_fill_rect(::org::nativescript::canvas::CanvasRenderingContext2D &context, float x, float y, float width, float height) noexcept;

void canvas_native_context_fill_text(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::rust::Str text, float x, float y, float width) noexcept;

::rust::Box<::org::nativescript::canvas::ImageData> canvas_native_context_get_image_data(::org::nativescript::canvas::CanvasRenderingContext2D &context, float sx, float sy, float sw, float sh) noexcept;

::rust::Box<::org::nativescript::canvas::Matrix> canvas_native_context_get_transform(::org::nativescript::canvas::CanvasRenderingContext2D &context) noexcept;

bool canvas_native_context_is_point_in_path(::org::nativescript::canvas::CanvasRenderingContext2D &context, float x, float y, ::rust::Str rule) noexcept;

bool canvas_native_context_is_point_in_path_with_path(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::org::nativescript::canvas::Path &path, float x, float y, ::rust::Str rule) noexcept;

bool canvas_native_context_is_point_in_stroke(::org::nativescript::canvas::CanvasRenderingContext2D &context, float x, float y) noexcept;

bool canvas_native_context_is_point_in_stroke_with_path(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::org::nativescript::canvas::Path &path, float x, float y) noexcept;

void canvas_native_context_line_to(::org::nativescript::canvas::CanvasRenderingContext2D &context, float x, float y) noexcept;

::rust::Box<::org::nativescript::canvas::TextMetrics> canvas_native_context_measure_text(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::rust::Str text) noexcept;

void canvas_native_context_move_to(::org::nativescript::canvas::CanvasRenderingContext2D &context, float x, float y) noexcept;

void canvas_native_context_put_image_data(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::org::nativescript::canvas::ImageData &image_data, float dx, float dy, float dirty_x, float dirty_y, float dirty_width, float dirty_height) noexcept;

void canvas_native_context_quadratic_curve_to(::org::nativescript::canvas::CanvasRenderingContext2D &context, float cpx, float cpy, float x, float y) noexcept;

void canvas_native_context_rect(::org::nativescript::canvas::CanvasRenderingContext2D &context, float x, float y, float width, float height) noexcept;

void canvas_native_context_round_rect(::org::nativescript::canvas::CanvasRenderingContext2D &context, float x, float y, float width, float height, ::rust::Slice<float const> radii) noexcept;

void canvas_native_context_round_rect_tl_tr_br_bl(::org::nativescript::canvas::CanvasRenderingContext2D &context, float x, float y, float width, float height, float top_left, float top_right, float bottom_right, float bottom_left) noexcept;

void canvas_native_context_reset_transform(::org::nativescript::canvas::CanvasRenderingContext2D &context) noexcept;

void canvas_native_context_restore(::org::nativescript::canvas::CanvasRenderingContext2D &context) noexcept;

void canvas_native_context_rotate(::org::nativescript::canvas::CanvasRenderingContext2D &context, float angle) noexcept;

void canvas_native_context_save(::org::nativescript::canvas::CanvasRenderingContext2D &context) noexcept;

void canvas_native_context_scale(::org::nativescript::canvas::CanvasRenderingContext2D &context, float x, float y) noexcept;

void canvas_native_context_set_transform(::org::nativescript::canvas::CanvasRenderingContext2D &context, float a, float b, float c, float d, float e, float f) noexcept;

void canvas_native_context_set_transform_matrix(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::org::nativescript::canvas::Matrix &matrix) noexcept;

void canvas_native_context_stroke(::org::nativescript::canvas::CanvasRenderingContext2D &context) noexcept;

void canvas_native_context_stroke_with_path(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::org::nativescript::canvas::Path &path) noexcept;

void canvas_native_context_stroke_rect(::org::nativescript::canvas::CanvasRenderingContext2D &context, float x, float y, float width, float height) noexcept;

void canvas_native_context_stroke_text(::org::nativescript::canvas::CanvasRenderingContext2D &context, ::rust::Str text, float x, float y, float width) noexcept;

void canvas_native_context_transform(::org::nativescript::canvas::CanvasRenderingContext2D &context, float a, float b, float c, float d, float e, float f) noexcept;

void canvas_native_context_translate(::org::nativescript::canvas::CanvasRenderingContext2D &context, float x, float y) noexcept;

::rust::Box<::org::nativescript::canvas::PaintStyle> canvas_native_context_create_pattern_webgl(::org::nativescript::canvas::WebGLState &source, ::org::nativescript::canvas::CanvasRenderingContext2D &context, ::rust::Str repetition) noexcept;

bool canvas_native_webgl_make_current(::org::nativescript::canvas::WebGLState &state) noexcept;

bool canvas_native_webgl_swap_buffers(::org::nativescript::canvas::WebGLState &state) noexcept;

::rust::String canvas_native_webgl_to_data_url(::org::nativescript::canvas::WebGLState &state, ::rust::Str format, ::std::int32_t quality) noexcept;

void canvas_native_webgl_resized(::org::nativescript::canvas::WebGLState &state) noexcept;

::rust::Str canvas_native_webgl_active_info_get_name(::org::nativescript::canvas::WebGLActiveInfo const &info) noexcept;

::std::int32_t canvas_native_webgl_active_info_get_size(::org::nativescript::canvas::WebGLActiveInfo const &info) noexcept;

::std::uint32_t canvas_native_webgl_active_info_get_type(::org::nativescript::canvas::WebGLActiveInfo const &info) noexcept;

bool canvas_native_webgl_active_info_get_is_empty(::org::nativescript::canvas::WebGLActiveInfo const &info) noexcept;

::std::int32_t canvas_native_webgl_shader_precision_format_get_range_min(::org::nativescript::canvas::WebGLShaderPrecisionFormat const &shader) noexcept;

::std::int32_t canvas_native_webgl_shader_precision_format_get_range_max(::org::nativescript::canvas::WebGLShaderPrecisionFormat const &shader) noexcept;

::std::int32_t canvas_native_webgl_shader_precision_format_get_precision(::org::nativescript::canvas::WebGLShaderPrecisionFormat const &shader) noexcept;

bool canvas_native_webgl_context_attribute_get_get_alpha(::org::nativescript::canvas::ContextAttributes const &attr) noexcept;

bool canvas_native_webgl_context_attribute_get_get_antialias(::org::nativescript::canvas::ContextAttributes const &attr) noexcept;

bool canvas_native_webgl_context_attribute_get_get_depth(::org::nativescript::canvas::ContextAttributes const &attr) noexcept;

bool canvas_native_webgl_context_attribute_get_get_fail_if_major_performance_caveat(::org::nativescript::canvas::ContextAttributes const &attr) noexcept;

::rust::String canvas_native_webgl_context_attribute_get_get_power_preference(::org::nativescript::canvas::ContextAttributes const &attr) noexcept;

bool canvas_native_webgl_context_attribute_get_get_premultiplied_alpha(::org::nativescript::canvas::ContextAttributes const &attr) noexcept;

bool canvas_native_webgl_context_attribute_get_get_preserve_drawing_buffer(::org::nativescript::canvas::ContextAttributes const &attr) noexcept;

bool canvas_native_webgl_context_attribute_get_get_stencil(::org::nativescript::canvas::ContextAttributes const &attr) noexcept;

bool canvas_native_webgl_context_attribute_get_get_desynchronized(::org::nativescript::canvas::ContextAttributes const &attr) noexcept;

bool canvas_native_webgl_context_attribute_get_get_xr_compatible(::org::nativescript::canvas::ContextAttributes const &attr) noexcept;

bool canvas_native_webgl_context_extension_is_none(::org::nativescript::canvas::WebGLExtension const &extension) noexcept;

::org::nativescript::canvas::WebGLExtensionType canvas_native_webgl_context_extension_get_type(::org::nativescript::canvas::WebGLExtension const &extension) noexcept;

::rust::Box<::org::nativescript::canvas::EXT_disjoint_timer_query> canvas_native_webgl_context_extension_to_ext_disjoint_timer_query(::rust::Box<::org::nativescript::canvas::WebGLExtension> extension) noexcept;

::rust::Box<::org::nativescript::canvas::ANGLE_instanced_arrays> canvas_native_webgl_context_extension_to_angle_instanced_arrays(::rust::Box<::org::nativescript::canvas::WebGLExtension> extension) noexcept;

::rust::Box<::org::nativescript::canvas::WEBGL_lose_context> canvas_native_webgl_context_extension_to_lose_context(::rust::Box<::org::nativescript::canvas::WebGLExtension> extension) noexcept;

::rust::Box<::org::nativescript::canvas::WEBGL_draw_buffers> canvas_native_webgl_context_extension_to_draw_buffers(::rust::Box<::org::nativescript::canvas::WebGLExtension> extension) noexcept;

::rust::Box<::org::nativescript::canvas::OES_vertex_array_object> canvas_native_webgl_context_extension_to_oes_vertex_array_object(::rust::Box<::org::nativescript::canvas::WebGLExtension> extension) noexcept;

void canvas_native_webgl_lose_context_lose_context(::org::nativescript::canvas::WEBGL_lose_context const &context) noexcept;

void canvas_native_webgl_lose_context_restore_context(::org::nativescript::canvas::WEBGL_lose_context const &context) noexcept;

void canvas_native_webgl_draw_buffers_draw_buffers_webgl(::rust::Slice<::std::uint32_t const> buffers, ::org::nativescript::canvas::WEBGL_draw_buffers const &context) noexcept;

::std::uint32_t canvas_native_webgl_oes_vertex_array_object_create_vertex_array_oes(::org::nativescript::canvas::OES_vertex_array_object const &object) noexcept;

void canvas_native_webgl_oes_vertex_array_object_delete_vertex_array_oes(::std::uint32_t array_object, ::org::nativescript::canvas::OES_vertex_array_object const &object) noexcept;

bool canvas_native_webgl_oes_vertex_array_object_is_vertex_array_oes(::std::uint32_t array_object, ::org::nativescript::canvas::OES_vertex_array_object const &object) noexcept;

void canvas_native_webgl_oes_vertex_array_object_bind_vertex_array_oes(::std::uint32_t array_object, ::org::nativescript::canvas::OES_vertex_array_object const &object) noexcept;

bool canvas_native_webgl_framebuffer_attachment_parameter_get_is_texture(::org::nativescript::canvas::WebGLFramebufferAttachmentParameter const &param) noexcept;

bool canvas_native_webgl_framebuffer_attachment_parameter_get_is_renderbuffer(::org::nativescript::canvas::WebGLFramebufferAttachmentParameter const &param) noexcept;

::std::int32_t canvas_native_webgl_framebuffer_attachment_parameter_get_value(::org::nativescript::canvas::WebGLFramebufferAttachmentParameter const &param) noexcept;

::org::nativescript::canvas::WebGLResultType canvas_native_webgl_result_get_type(::org::nativescript::canvas::WebGLResult const &result) noexcept;

bool canvas_native_webgl_result_get_bool(::org::nativescript::canvas::WebGLResult const &result) noexcept;

::rust::Vec<::std::int32_t> canvas_native_webgl_result_get_i32_array(::org::nativescript::canvas::WebGLResult const &result) noexcept;

::rust::Vec<::std::uint32_t> canvas_native_webgl_result_get_u32_array(::org::nativescript::canvas::WebGLResult const &result) noexcept;

::rust::Vec<float> canvas_native_webgl_result_get_f32_array(::org::nativescript::canvas::WebGLResult const &result) noexcept;

::rust::Vec<::std::uint8_t> canvas_native_webgl_result_get_bool_array(::org::nativescript::canvas::WebGLResult const &result) noexcept;

::std::uint32_t canvas_native_webgl_result_get_u32(::org::nativescript::canvas::WebGLResult const &result) noexcept;

::std::int32_t canvas_native_webgl_result_get_i32(::org::nativescript::canvas::WebGLResult const &result) noexcept;

float canvas_native_webgl_result_get_f32(::org::nativescript::canvas::WebGLResult const &result) noexcept;

::rust::String canvas_native_webgl_result_get_string(::org::nativescript::canvas::WebGLResult const &result) noexcept;

bool canvas_native_webgl_result_get_is_none(::org::nativescript::canvas::WebGLResult const &result) noexcept;

::std::int32_t canvas_native_webgl_state_get_unpack_colorspace_conversion_webgl(::org::nativescript::canvas::WebGLState &state) noexcept;

bool canvas_native_webgl_state_get_flip_y(::org::nativescript::canvas::WebGLState &state) noexcept;

bool canvas_native_webgl_state_get_premultiplied_alpha(::org::nativescript::canvas::WebGLState &state) noexcept;

::std::int32_t canvas_native_webgl_state_get_drawing_buffer_width(::org::nativescript::canvas::WebGLState &state) noexcept;

::std::int32_t canvas_native_webgl_state_get_drawing_buffer_height(::org::nativescript::canvas::WebGLState &state) noexcept;

::std::uint32_t canvas_native_webgl_ext_disjoint_timer_query_create_query_ext(::org::nativescript::canvas::EXT_disjoint_timer_query const &query) noexcept;

void canvas_native_webgl_ext_disjoint_timer_query_delete_query_ext(::std::uint32_t value, ::org::nativescript::canvas::EXT_disjoint_timer_query const &query) noexcept;

bool canvas_native_webgl_ext_disjoint_timer_query_is_query_ext(::std::uint32_t value, ::org::nativescript::canvas::EXT_disjoint_timer_query const &query) noexcept;

void canvas_native_webgl_ext_disjoint_timer_query_begin_query_ext(::std::uint32_t target, ::std::uint32_t value, ::org::nativescript::canvas::EXT_disjoint_timer_query const &query) noexcept;

void canvas_native_webgl_ext_disjoint_timer_query_end_query_ext(::std::uint32_t target, ::org::nativescript::canvas::EXT_disjoint_timer_query const &query) noexcept;

void canvas_native_webgl_ext_disjoint_timer_query_query_counter_ext(::std::uint32_t value, ::std::uint32_t target, ::org::nativescript::canvas::EXT_disjoint_timer_query const &query) noexcept;

::std::int32_t canvas_native_webgl_ext_disjoint_timer_query_get_query_ext(::std::uint32_t target, ::std::uint32_t pname, ::org::nativescript::canvas::EXT_disjoint_timer_query const &query) noexcept;

::rust::Box<::org::nativescript::canvas::WebGLResult> canvas_native_webgl_ext_disjoint_timer_query_get_query_object_ext(::std::uint32_t target, ::std::uint32_t pname, ::org::nativescript::canvas::EXT_disjoint_timer_query const &query) noexcept;

void canvas_native_webgl_angle_instanced_arrays_draw_arrays_instanced_angle(::std::uint32_t mode, ::std::int32_t first, ::std::int32_t count, ::std::int32_t primcount, ::org::nativescript::canvas::ANGLE_instanced_arrays const &arrays) noexcept;

void canvas_native_webgl_angle_instanced_arrays_draw_elements_instanced_angle(::std::uint32_t mode, ::std::int32_t count, ::std::uint32_t type_, ::std::int32_t offset, ::std::int32_t primcount, ::org::nativescript::canvas::ANGLE_instanced_arrays const &arrays) noexcept;

void canvas_native_webgl_angle_instanced_arrays_vertex_attrib_divisor_angle(::std::uint32_t index, ::std::uint32_t divisor, ::org::nativescript::canvas::ANGLE_instanced_arrays const &arrays) noexcept;

::rust::Box<::org::nativescript::canvas::WebGLState> canvas_native_webgl_create(::std::int64_t gl_context, ::rust::Str version, bool alpha, bool antialias, bool depth, bool fail_if_major_performance_caveat, ::rust::Str power_preference, bool premultiplied_alpha, bool preserve_drawing_buffer, bool stencil, bool desynchronized, bool xr_compatible) noexcept;

::rust::Box<::org::nativescript::canvas::WebGLState> canvas_native_webgl_create_no_window(::std::int32_t width, ::std::int32_t height, ::rust::Str version, bool alpha, bool antialias, bool depth, bool fail_if_major_performance_caveat, ::rust::Str power_preference, bool premultiplied_alpha, bool preserve_drawing_buffer, bool stencil, bool desynchronized, bool xr_compatible, bool is_canvas) noexcept;

void canvas_native_webgl_active_texture(::std::uint32_t texture, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_attach_shader(::std::uint32_t program, ::std::uint32_t shader, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_bind_attrib_location(::std::uint32_t program, ::std::uint32_t index, ::rust::Str name, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_bind_buffer(::std::uint32_t target, ::std::uint32_t buffer, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_bind_frame_buffer(::std::uint32_t target, ::std::uint32_t framebuffer, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_bind_render_buffer(::std::uint32_t target, ::std::uint32_t renderbuffer, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_bind_texture(::std::uint32_t target, ::std::uint32_t texture, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_blend_color(float red, float green, float blue, float alpha, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_blend_equation_separate(::std::uint32_t mode_rgb, ::std::uint32_t mode_alpha, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_blend_equation(::std::uint32_t mode, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_blend_func_separate(::std::uint32_t src_rgb, ::std::uint32_t dst_rgb, ::std::uint32_t src_alpha, ::std::uint32_t dst_alpha, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_blend_func(::std::uint32_t sfactor, ::std::uint32_t dfactor, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_buffer_data(::std::uint32_t target, ::rust::Slice<::std::uint8_t const> src_data, ::std::uint32_t usage, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_buffer_data_u16(::std::uint32_t target, ::rust::Slice<::std::uint16_t const> src_data, ::std::uint32_t usage, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_buffer_data_f32(::std::uint32_t target, ::rust::Slice<float const> src_data, ::std::uint32_t usage, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_buffer_data_none(::std::uint32_t target, ::rust::isize size, ::std::uint32_t usage, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_buffer_sub_data(::std::uint32_t target, ::rust::isize offset, ::rust::Slice<::std::uint8_t const> src_data, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_buffer_sub_data_none(::std::uint32_t target, ::rust::isize offset, ::org::nativescript::canvas::WebGLState &state) noexcept;

::std::uint32_t canvas_native_webgl_check_frame_buffer_status(::std::uint32_t target, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_clear(::std::uint32_t mask, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_clear_color(float red, float green, float blue, float alpha, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_clear_depth(float depth, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_clear_stencil(::std::int32_t stencil, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_color_mask(bool red, bool green, bool blue, bool alpha, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_commit(::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_compile_shader(::std::uint32_t shader, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_compressed_tex_image2d(::std::uint32_t target, ::std::int32_t level, ::std::uint32_t internalformat, ::std::int32_t width, ::std::int32_t height, ::std::int32_t border, ::rust::Slice<::std::uint8_t const> pixels, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_compressed_tex_image2d_none(::std::uint32_t target, ::std::int32_t level, ::std::uint32_t internalformat, ::std::int32_t width, ::std::int32_t height, ::std::int32_t border, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_compressed_tex_sub_image2d(::std::uint32_t target, ::std::int32_t level, ::std::int32_t xoffset, ::std::int32_t yoffset, ::std::int32_t width, ::std::int32_t height, ::std::uint32_t format, ::rust::Slice<::std::uint8_t const> pixels, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_copy_tex_image2d(::std::uint32_t target, ::std::int32_t level, ::std::uint32_t internalformat, ::std::int32_t x, ::std::int32_t y, ::std::int32_t width, ::std::int32_t height, ::std::int32_t border, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_copy_tex_sub_image2d(::std::uint32_t target, ::std::int32_t level, ::std::int32_t xoffset, ::std::int32_t yoffset, ::std::int32_t x, ::std::int32_t y, ::std::int32_t width, ::std::int32_t height, ::org::nativescript::canvas::WebGLState &state) noexcept;

::std::uint32_t canvas_native_webgl_create_buffer(::org::nativescript::canvas::WebGLState &state) noexcept;

::std::uint32_t canvas_native_webgl_create_framebuffer(::org::nativescript::canvas::WebGLState &state) noexcept;

::std::uint32_t canvas_native_webgl_create_program(::org::nativescript::canvas::WebGLState &state) noexcept;

::std::uint32_t canvas_native_webgl_create_renderbuffer(::org::nativescript::canvas::WebGLState &state) noexcept;

::std::uint32_t canvas_native_webgl_create_shader(::std::uint32_t shader_type, ::org::nativescript::canvas::WebGLState &state) noexcept;

::std::uint32_t canvas_native_webgl_create_texture(::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_cull_face(::std::uint32_t mode, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_delete_buffer(::std::uint32_t buffer, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_delete_framebuffer(::std::uint32_t frame_buffer, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_delete_program(::std::uint32_t program, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_delete_renderbuffer(::std::uint32_t render_buffer, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_delete_shader(::std::uint32_t shader, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_delete_texture(::std::uint32_t texture, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_depth_func(::std::uint32_t func, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_depth_mask(bool flag, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_depth_range(float z_near, float z_far, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_detach_shader(::std::uint32_t program, ::std::uint32_t shader, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_disable(::std::uint32_t cap, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_disable_vertex_attrib_array(::std::uint32_t index, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_draw_arrays(::std::uint32_t mode, ::std::int32_t first, ::std::int32_t count, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_draw_elements(::std::uint32_t mode, ::std::int32_t count, ::std::uint32_t element_type, ::rust::isize offset, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_enable(::std::uint32_t cap, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_enable_vertex_attrib_array(::std::uint32_t index, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_finish(::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_flush(::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_framebuffer_renderbuffer(::std::uint32_t target, ::std::uint32_t attachment, ::std::uint32_t renderbuffertarget, ::std::uint32_t renderbuffer, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_framebuffer_texture2d(::std::uint32_t target, ::std::uint32_t attachment, ::std::uint32_t textarget, ::std::uint32_t texture, ::std::int32_t level, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_front_face(::std::uint32_t mode, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_generate_mipmap(::std::uint32_t target, ::org::nativescript::canvas::WebGLState &state) noexcept;

::rust::Box<::org::nativescript::canvas::WebGLActiveInfo> canvas_native_webgl_get_active_attrib(::std::uint32_t program, ::std::uint32_t index, ::org::nativescript::canvas::WebGLState &state) noexcept;

::rust::Box<::org::nativescript::canvas::WebGLActiveInfo> canvas_native_webgl_get_active_uniform(::std::uint32_t program, ::std::uint32_t index, ::org::nativescript::canvas::WebGLState &state) noexcept;

::rust::Vec<::std::uint32_t> canvas_native_webgl_get_attached_shaders(::std::uint32_t program, ::org::nativescript::canvas::WebGLState &state) noexcept;

::std::int32_t canvas_native_webgl_get_attrib_location(::std::uint32_t program, ::rust::Str name, ::org::nativescript::canvas::WebGLState &state) noexcept;

::std::int32_t canvas_native_webgl_get_buffer_parameter(::std::uint32_t target, ::std::uint32_t pname, ::org::nativescript::canvas::WebGLState &state) noexcept;

::rust::Box<::org::nativescript::canvas::ContextAttributes> canvas_native_webgl_get_context_attributes(::org::nativescript::canvas::WebGLState &state) noexcept;

::std::uint32_t canvas_native_webgl_get_error(::org::nativescript::canvas::WebGLState &state) noexcept;

::rust::Box<::org::nativescript::canvas::WebGLExtension> canvas_native_webgl_get_extension(::rust::Str name, ::org::nativescript::canvas::WebGLState &state) noexcept;

::rust::Box<::org::nativescript::canvas::WebGLFramebufferAttachmentParameter> canvas_native_webgl_get_framebuffer_attachment_parameter(::std::uint32_t target, ::std::uint32_t attachment, ::std::uint32_t pname, ::org::nativescript::canvas::WebGLState &state) noexcept;

::rust::Box<::org::nativescript::canvas::WebGLResult> canvas_native_webgl_get_parameter(::std::uint32_t pname, ::org::nativescript::canvas::WebGLState &state) noexcept;

::rust::String canvas_native_webgl_get_program_info_log(::std::uint32_t program, ::org::nativescript::canvas::WebGLState &state) noexcept;

::rust::Box<::org::nativescript::canvas::WebGLResult> canvas_native_webgl_get_program_parameter(::std::uint32_t program, ::std::uint32_t pname, ::org::nativescript::canvas::WebGLState &state) noexcept;

::std::int32_t canvas_native_webgl_get_renderbuffer_parameter(::std::uint32_t target, ::std::uint32_t pname, ::org::nativescript::canvas::WebGLState &state) noexcept;

::rust::String canvas_native_webgl_get_shader_info_log(::std::uint32_t shader, ::org::nativescript::canvas::WebGLState &state) noexcept;

::rust::Box<::org::nativescript::canvas::WebGLResult> canvas_native_webgl_get_shader_parameter(::std::uint32_t shader, ::std::uint32_t pname, ::org::nativescript::canvas::WebGLState &state) noexcept;

::rust::Box<::org::nativescript::canvas::WebGLShaderPrecisionFormat> canvas_native_webgl_get_shader_precision_format(::std::uint32_t shader_type, ::std::uint32_t precision_type, ::org::nativescript::canvas::WebGLState &state) noexcept;

::rust::String canvas_native_webgl_get_shader_source(::std::uint32_t shader, ::org::nativescript::canvas::WebGLState &state) noexcept;

::rust::Vec<::rust::String> canvas_native_webgl_get_supported_extensions(::org::nativescript::canvas::WebGLState &state) noexcept;

::rust::String canvas_native_webgl_get_supported_extensions_to_string(::org::nativescript::canvas::WebGLState &state) noexcept;

::std::int32_t canvas_native_webgl_get_tex_parameter(::std::uint32_t target, ::std::uint32_t pname, ::org::nativescript::canvas::WebGLState &state) noexcept;

::std::int32_t canvas_native_webgl_get_uniform_location(::std::uint32_t program, ::rust::Str name, ::org::nativescript::canvas::WebGLState &state) noexcept;

::rust::Box<::org::nativescript::canvas::WebGLResult> canvas_native_webgl_get_uniform(::std::uint32_t program, ::std::int32_t location, ::org::nativescript::canvas::WebGLState &state) noexcept;

::std::size_t canvas_native_webgl_get_vertex_attrib_offset(::std::uint32_t index, ::std::uint32_t pname, ::org::nativescript::canvas::WebGLState &state) noexcept;

::rust::Box<::org::nativescript::canvas::WebGLResult> canvas_native_webgl_get_vertex_attrib(::std::uint32_t index, ::std::uint32_t pname, ::org::nativescript::canvas::WebGLState &state) noexcept;

bool canvas_native_webgl_get_is_context_lost(::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_hint(::std::uint32_t target, ::std::uint32_t mode, ::org::nativescript::canvas::WebGLState &state) noexcept;

bool canvas_native_webgl_is_buffer(::std::uint32_t buffer, ::org::nativescript::canvas::WebGLState &state) noexcept;

bool canvas_native_webgl_is_enabled(::std::uint32_t cap, ::org::nativescript::canvas::WebGLState &state) noexcept;

bool canvas_native_webgl_is_framebuffer(::std::uint32_t framebuffer, ::org::nativescript::canvas::WebGLState &state) noexcept;

bool canvas_native_webgl_is_program(::std::uint32_t program, ::org::nativescript::canvas::WebGLState &state) noexcept;

bool canvas_native_webgl_is_renderbuffer(::std::uint32_t renderbuffer, ::org::nativescript::canvas::WebGLState &state) noexcept;

bool canvas_native_webgl_is_shader(::std::uint32_t shader, ::org::nativescript::canvas::WebGLState &state) noexcept;

bool canvas_native_webgl_is_texture(::std::uint32_t texture, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_line_width(float width, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_link_program(::std::uint32_t program, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_pixel_storei(::std::uint32_t pname, ::std::int32_t param, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_polygon_offset(float factor, float units, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_read_pixels_u8(::std::int32_t x, ::std::int32_t y, ::std::int32_t width, ::std::int32_t height, ::std::uint32_t format, ::std::uint32_t pixel_type, ::rust::Slice<::std::uint8_t > pixels, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_read_pixels_u16(::std::int32_t x, ::std::int32_t y, ::std::int32_t width, ::std::int32_t height, ::std::uint32_t format, ::std::uint32_t pixel_type, ::rust::Slice<::std::uint16_t > pixels, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_read_pixels_f32(::std::int32_t x, ::std::int32_t y, ::std::int32_t width, ::std::int32_t height, ::std::uint32_t format, ::std::uint32_t pixel_type, ::rust::Slice<float > pixels, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_renderbuffer_storage(::std::uint32_t target, ::std::uint32_t internal_format, ::std::int32_t width, ::std::int32_t height, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_sample_coverage(float value, bool invert, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_scissor(::std::int32_t x, ::std::int32_t y, ::std::int32_t width, ::std::int32_t height, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_shader_source(::std::uint32_t shader, ::rust::Str source, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_stencil_func(::std::uint32_t func, ::std::int32_t reference, ::std::uint32_t mask, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_stencil_func_separate(::std::uint32_t face, ::std::uint32_t func, ::std::int32_t reference, ::std::uint32_t mask, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_stencil_mask(::std::uint32_t mask, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_stencil_mask_separate(::std::uint32_t face, ::std::uint32_t mask, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_stencil_op(::std::uint32_t fail, ::std::uint32_t zfail, ::std::uint32_t zpass, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_stencil_op_separate(::std::uint32_t face, ::std::uint32_t fail, ::std::uint32_t zfail, ::std::uint32_t zpass, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_tex_image2d_image_none(::std::int32_t target, ::std::int32_t level, ::std::int32_t internalformat, ::std::int32_t format, ::std::int32_t image_type, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_tex_image2d_canvas2d(::std::int32_t target, ::std::int32_t level, ::std::int32_t internalformat, ::std::int32_t format, ::std::int32_t image_type, ::org::nativescript::canvas::CanvasRenderingContext2D &canvas, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_tex_image2d_webgl(::std::int32_t target, ::std::int32_t level, ::std::int32_t internalformat, ::std::int32_t format, ::std::int32_t image_type, ::org::nativescript::canvas::WebGLState &webgl, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_tex_image2d(::std::int32_t target, ::std::int32_t level, ::std::int32_t internalformat, ::std::int32_t width, ::std::int32_t height, ::std::int32_t border, ::std::int32_t format, ::std::int32_t image_type, ::rust::Slice<::std::uint8_t > buf, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_tex_image2d_none(::std::int32_t target, ::std::int32_t level, ::std::int32_t internalformat, ::std::int32_t width, ::std::int32_t height, ::std::int32_t border, ::std::int32_t format, ::std::int32_t image_type, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_tex_image2d_image_asset(::std::int32_t target, ::std::int32_t level, ::std::int32_t internalformat, ::std::int32_t format, ::std::int32_t image_type, ::org::nativescript::canvas::ImageAsset &image_asset, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_tex_parameterf(::std::uint32_t target, ::std::uint32_t pname, float param, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_tex_parameteri(::std::uint32_t target, ::std::uint32_t pname, ::std::int32_t param, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_tex_sub_image2d_asset(::std::uint32_t target, ::std::int32_t level, ::std::int32_t xoffset, ::std::int32_t yoffset, ::std::uint32_t format, ::std::int32_t image_type, ::org::nativescript::canvas::ImageAsset &asset, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_tex_sub_image2d(::std::uint32_t target, ::std::int32_t level, ::std::int32_t xoffset, ::std::int32_t yoffset, ::std::int32_t width, ::std::int32_t height, ::std::uint32_t format, ::std::int32_t image_type, ::rust::Slice<::std::uint8_t const> buf, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_uniform1f(::std::int32_t location, float v0, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_uniform1fv(::std::int32_t location, ::rust::Slice<float const> value, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_uniform1i(::std::int32_t location, ::std::int32_t v0, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_uniform1iv(::std::int32_t location, ::rust::Slice<::std::int32_t const> value, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_uniform2f(::std::int32_t location, float v0, float v1, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_uniform2fv(::std::int32_t location, ::rust::Slice<float const> value, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_uniform2i(::std::int32_t location, ::std::int32_t v0, ::std::int32_t v1, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_uniform2iv(::std::int32_t location, ::rust::Slice<::std::int32_t const> value, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_uniform3f(::std::int32_t location, float v0, float v1, float v2, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_uniform3fv(::std::int32_t location, ::rust::Slice<float const> value, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_uniform3i(::std::int32_t location, ::std::int32_t v0, ::std::int32_t v1, ::std::int32_t v2, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_uniform3iv(::std::int32_t location, ::rust::Slice<::std::int32_t const> value, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_uniform4f(::std::int32_t location, float v0, float v1, float v2, float v3, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_uniform4fv(::std::int32_t location, ::rust::Slice<float const> value, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_uniform4i(::std::int32_t location, ::std::int32_t v0, ::std::int32_t v1, ::std::int32_t v2, ::std::int32_t v3, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_uniform4iv(::std::int32_t location, ::rust::Slice<::std::int32_t const> value, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_uniform_matrix2fv(::std::int32_t location, bool transpose, ::rust::Slice<float const> value, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_uniform_matrix3fv(::std::int32_t location, bool transpose, ::rust::Slice<float const> value, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_uniform_matrix4fv(::std::int32_t location, bool transpose, ::rust::Slice<float const> value, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_use_program(::std::uint32_t program, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_validate_program(::std::uint32_t program, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_vertex_attrib1f(::std::uint32_t index, float v0, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_vertex_attrib1fv(::std::uint32_t index, ::rust::Slice<float const> value, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_vertex_attrib2f(::std::uint32_t index, float v0, float v1, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_vertex_attrib2fv(::std::uint32_t index, ::rust::Slice<float const> value, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_vertex_attrib3f(::std::uint32_t index, float v0, float v1, float v2, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_vertex_attrib3fv(::std::uint32_t index, ::rust::Slice<float const> value, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_vertex_attrib4f(::std::uint32_t index, float v0, float v1, float v2, float v3, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_vertex_attrib4fv(::std::uint32_t index, ::rust::Slice<float const> value, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_vertex_attrib_pointer(::std::uint32_t index, ::std::int32_t size, ::std::uint32_t d_type, bool normalized, ::std::int32_t stride, ::rust::isize offset, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl_viewport(::std::int32_t x, ::std::int32_t y, ::std::int32_t width, ::std::int32_t height, ::org::nativescript::canvas::WebGLState &state) noexcept;

::rust::isize canvas_native_webgl2_indexed_parameter_get_value(::org::nativescript::canvas::WebGLIndexedParameter const &param) noexcept;

::rust::isize canvas_native_webgl2_indexed_parameter_get_buffer_value(::org::nativescript::canvas::WebGLIndexedParameter const &param) noexcept;

bool canvas_native_webgl2_indexed_parameter_get_is_buffer(::org::nativescript::canvas::WebGLIndexedParameter const &param) noexcept;

void canvas_native_webgl2_begin_query(::std::uint32_t target, ::std::uint32_t id, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl2_begin_transform_feedback(::std::uint32_t primitive_mode, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl2_bind_buffer_base(::std::uint32_t target, ::std::uint32_t index, ::std::uint32_t buffer, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl2_bind_buffer_range(::std::uint32_t target, ::std::uint32_t index, ::std::uint32_t buffer, ::rust::isize offset, ::rust::isize size, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl2_bind_sampler(::std::uint32_t unit, ::std::uint32_t sampler, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl2_bind_transform_feedback(::std::uint32_t target, ::std::uint32_t transform_feedback, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl2_bind_vertex_array(::std::uint32_t vertex_array, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl2_blit_framebuffer(::std::int32_t src_x0, ::std::int32_t src_y0, ::std::int32_t src_x1, ::std::int32_t src_y1, ::std::int32_t dst_x0, ::std::int32_t dst_y0, ::std::int32_t dst_x1, ::std::int32_t dst_y1, ::std::uint32_t mask, ::std::uint32_t filter, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl2_clear_bufferfi(::std::uint32_t buffer, ::std::int32_t drawbuffer, float depth, ::std::int32_t stencil, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl2_clear_bufferfv(::std::uint32_t buffer, ::std::int32_t drawbuffer, ::rust::Slice<float const> values, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl2_clear_bufferiv(::std::uint32_t buffer, ::std::int32_t drawbuffer, ::rust::Slice<::std::int32_t const> values, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl2_clear_bufferuiv(::std::uint32_t buffer, ::std::int32_t drawbuffer, ::rust::Slice<::std::uint32_t const> values, ::org::nativescript::canvas::WebGLState &state) noexcept;

::std::uint32_t canvas_native_webgl2_client_wait_sync(::org::nativescript::canvas::WebGLSync const &sync, ::std::uint32_t flags, ::rust::isize timeout, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl2_compressed_tex_sub_image3d_none(::std::uint32_t target, ::std::int32_t level, ::std::int32_t xoffset, ::std::int32_t yoffset, ::std::int32_t zoffset, ::std::int32_t width, ::std::int32_t height, ::std::int32_t depth, ::std::uint32_t format, ::std::int32_t image_size, ::std::int32_t offset, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl2_compressed_tex_sub_image3d(::std::uint32_t target, ::std::int32_t level, ::std::int32_t xoffset, ::std::int32_t yoffset, ::std::int32_t zoffset, ::std::int32_t width, ::std::int32_t height, ::std::int32_t depth, ::std::uint32_t format, ::rust::Slice<::std::uint8_t const> src, ::std::size_t src_offset, ::std::size_t src_length_override, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl2_copy_buffer_sub_data(::std::uint32_t read_target, ::std::uint32_t write_target, ::rust::isize read_offset, ::rust::isize write_offset, ::rust::isize size, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl2_copy_tex_sub_image3d(::std::uint32_t target, ::std::int32_t level, ::std::int32_t xoffset, ::std::int32_t yoffset, ::std::int32_t zoffset, ::std::int32_t x, ::std::int32_t y, ::std::int32_t width, ::std::int32_t height, ::org::nativescript::canvas::WebGLState &state) noexcept;

::std::uint32_t canvas_native_webgl2_create_query(::org::nativescript::canvas::WebGLState &state) noexcept;

::std::uint32_t canvas_native_webgl2_create_sampler(::org::nativescript::canvas::WebGLState &state) noexcept;

::std::uint32_t canvas_native_webgl2_create_transform_feedback(::org::nativescript::canvas::WebGLState &state) noexcept;

::std::uint32_t canvas_native_webgl2_create_vertex_array(::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl2_delete_query_with_query(::std::uint32_t id, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl2_delete_sampler_with_sampler(::std::uint32_t sampler, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl2_delete_sync_with_sync(::org::nativescript::canvas::WebGLSync const &sync, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl2_delete_transform_feedback(::std::uint32_t transform_feedback, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl2_delete_vertex_array_with_vertex_array(::std::uint32_t vertex_array, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl2_draw_arrays_instanced(::std::uint32_t mode, ::std::int32_t first, ::std::int32_t count, ::std::int32_t instance_count, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl2_draw_buffers(::rust::Slice<::std::uint32_t const> buffers, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl2_draw_elements_instanced(::std::uint32_t mode, ::std::int32_t count, ::std::uint32_t type_, ::rust::isize offset, ::std::int32_t instance_count, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl2_draw_range_elements(::std::uint32_t mode, ::std::uint32_t start, ::std::uint32_t end, ::std::int32_t count, ::std::uint32_t type_, ::rust::isize offset, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl2_end_query(::std::uint32_t target, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl2_end_transform_feedback(::org::nativescript::canvas::WebGLState &state) noexcept;

::rust::Box<::org::nativescript::canvas::WebGLSync> canvas_native_webgl2_fence_sync(::std::uint32_t condition, ::std::uint32_t flags, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl2_framebuffer_texture_layer(::std::uint32_t target, ::std::uint32_t attachment, ::std::uint32_t texture, ::std::int32_t level, ::std::int32_t layer, ::org::nativescript::canvas::WebGLState &state) noexcept;

::rust::String canvas_native_webgl2_get_active_uniform_block_name(::std::uint32_t program, ::std::uint32_t uniform_block_index, ::org::nativescript::canvas::WebGLState &state) noexcept;

::rust::Box<::org::nativescript::canvas::WebGLResult> canvas_native_webgl2_get_active_uniform_block_parameter(::std::uint32_t program, ::std::uint32_t uniform_block_index, ::std::uint32_t pname, ::org::nativescript::canvas::WebGLState &state) noexcept;

::rust::Box<::org::nativescript::canvas::WebGLResult> canvas_native_webgl2_get_active_uniforms(::std::uint32_t program, ::rust::Slice<::std::uint32_t const> uniform_indices, ::std::uint32_t pname, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl2_get_buffer_sub_data(::std::uint32_t target, ::rust::isize src_byte_offset, ::rust::Slice<::std::uint8_t > dst_data, ::std::size_t dst_offset, ::std::size_t length, ::org::nativescript::canvas::WebGLState &state) noexcept;

::std::int32_t canvas_native_webgl2_get_frag_data_location(::std::uint32_t program, ::rust::Str name, ::org::nativescript::canvas::WebGLState &state) noexcept;

::rust::Box<::org::nativescript::canvas::WebGLIndexedParameter> canvas_native_webgl2_get_indexed_parameter(::std::uint32_t target, ::std::uint32_t index, ::org::nativescript::canvas::WebGLState &state) noexcept;

::rust::Box<::org::nativescript::canvas::WebGLResult> canvas_native_webgl2_get_internalformat_parameter(::std::uint32_t target, ::std::uint32_t internalformat, ::std::uint32_t pname, ::org::nativescript::canvas::WebGLState &state) noexcept;

::rust::Box<::org::nativescript::canvas::WebGLResult> canvas_native_webgl2_get_parameter(::std::uint32_t pname, ::org::nativescript::canvas::WebGLState &state) noexcept;

::rust::Box<::org::nativescript::canvas::WebGLResult> canvas_native_webgl2_get_query_parameter(::std::uint32_t query, ::std::uint32_t pname, ::org::nativescript::canvas::WebGLState &state) noexcept;

::rust::Box<::org::nativescript::canvas::WebGLResult> canvas_native_webgl2_get_query(::std::uint32_t target, ::std::uint32_t pname, ::org::nativescript::canvas::WebGLState &state) noexcept;

::rust::Box<::org::nativescript::canvas::WebGLResult> canvas_native_webgl2_get_sampler_parameter(::std::uint32_t sampler, ::std::uint32_t pname, ::org::nativescript::canvas::WebGLState &state) noexcept;

::rust::Box<::org::nativescript::canvas::WebGLResult> canvas_native_webgl2_get_sync_parameter(::org::nativescript::canvas::WebGLSync const &sync, ::std::uint32_t pname, ::org::nativescript::canvas::WebGLState &state) noexcept;

::rust::Box<::org::nativescript::canvas::WebGLActiveInfo> canvas_native_webgl2_get_transform_feedback_varying(::std::uint32_t program, ::std::uint32_t index, ::org::nativescript::canvas::WebGLState &state) noexcept;

::std::uint32_t canvas_native_webgl2_get_uniform_block_index(::std::uint32_t program, ::rust::Str uniform_block_name, ::org::nativescript::canvas::WebGLState &state) noexcept;

::rust::Vec<::std::uint32_t> canvas_native_webgl2_get_uniform_indices(::std::uint32_t program, ::rust::Slice<::rust::Str const> uniform_names, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl2_invalidate_framebuffer(::std::uint32_t target, ::rust::Slice<::std::uint32_t const> attachments, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl2_invalidate_sub_framebuffer(::std::uint32_t target, ::rust::Slice<::std::uint32_t const> attachments, ::std::int32_t x, ::std::int32_t y, ::std::int32_t width, ::std::int32_t height, ::org::nativescript::canvas::WebGLState &state) noexcept;

bool canvas_native_webgl2_is_query(::std::uint32_t query, ::org::nativescript::canvas::WebGLState &state) noexcept;

bool canvas_native_webgl2_is_sampler(::std::uint32_t sampler, ::org::nativescript::canvas::WebGLState &state) noexcept;

bool canvas_native_webgl2_is_sync(::org::nativescript::canvas::WebGLSync const &sync, ::org::nativescript::canvas::WebGLState &state) noexcept;

bool canvas_native_webgl2_is_transform_feedback(::std::uint32_t transform_feedback, ::org::nativescript::canvas::WebGLState &state) noexcept;

bool canvas_native_webgl2_is_vertex_array(::std::uint32_t vertex_array, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl2_pause_transform_feedback(::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl2_read_buffer(::std::uint32_t src, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl2_renderbuffer_storage_multisample(::std::uint32_t target, ::std::int32_t samples, ::std::uint32_t internal_format, ::std::int32_t width, ::std::int32_t height, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl2_resume_transform_feedback(::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl2_sampler_parameterf(::std::uint32_t sampler, ::std::uint32_t pname, float param, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl2_sampler_parameteri(::std::uint32_t sampler, ::std::uint32_t pname, ::std::int32_t param, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl2_tex_image3d_none(::std::uint32_t target, ::std::int32_t level, ::std::int32_t internalformat, ::std::int32_t width, ::std::int32_t height, ::std::int32_t depth, ::std::int32_t border, ::std::uint32_t format, ::std::uint32_t type_, ::std::size_t offset, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl2_tex_image3d_asset(::std::uint32_t target, ::std::int32_t level, ::std::int32_t internalformat, ::std::int32_t width, ::std::int32_t height, ::std::int32_t depth, ::std::int32_t border, ::std::uint32_t format, ::std::uint32_t type_, ::org::nativescript::canvas::ImageAsset const &asset, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl2_tex_image3d(::std::uint32_t target, ::std::int32_t level, ::std::int32_t internalformat, ::std::int32_t width, ::std::int32_t height, ::std::int32_t depth, ::std::int32_t border, ::std::uint32_t format, ::std::uint32_t type_, ::rust::Slice<::std::uint8_t const> buf, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl2_tex_image3d_offset(::std::uint32_t target, ::std::int32_t level, ::std::int32_t internalformat, ::std::int32_t width, ::std::int32_t height, ::std::int32_t depth, ::std::int32_t border, ::std::uint32_t format, ::std::uint32_t type_, ::rust::Slice<::std::uint8_t const> buf, ::std::size_t offset, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl2_tex_storage2d(::std::uint32_t target, ::std::int32_t levels, ::std::uint32_t internalformat, ::std::int32_t width, ::std::int32_t height, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl2_tex_storage3d(::std::uint32_t target, ::std::int32_t levels, ::std::uint32_t internalformat, ::std::int32_t width, ::std::int32_t height, ::std::int32_t depth, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl2_tex_sub_image3d_none(::std::uint32_t target, ::std::int32_t level, ::std::int32_t xoffset, ::std::int32_t yoffset, ::std::int32_t zoffset, ::std::int32_t width, ::std::int32_t height, ::std::int32_t depth, ::std::uint32_t format, ::std::uint32_t type_, ::std::size_t offset, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl2_tex_sub_image3d(::std::uint32_t target, ::std::int32_t level, ::std::int32_t xoffset, ::std::int32_t yoffset, ::std::int32_t zoffset, ::std::int32_t width, ::std::int32_t height, ::std::int32_t depth, ::std::uint32_t format, ::std::uint32_t type_, ::rust::Slice<::std::uint8_t const> buf, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl2_tex_sub_image3d_asset(::std::uint32_t target, ::std::int32_t level, ::std::int32_t xoffset, ::std::int32_t yoffset, ::std::int32_t zoffset, ::std::int32_t width, ::std::int32_t height, ::std::int32_t depth, ::std::uint32_t format, ::std::uint32_t type_, ::org::nativescript::canvas::ImageAsset const &asset, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl2_tex_sub_image3d_offset(::std::uint32_t target, ::std::int32_t level, ::std::int32_t xoffset, ::std::int32_t yoffset, ::std::int32_t zoffset, ::std::int32_t width, ::std::int32_t height, ::std::int32_t depth, ::std::uint32_t format, ::std::uint32_t type_, ::rust::Slice<::std::uint8_t const> buf, ::std::size_t offset, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl2_transform_feedback_varyings(::std::uint32_t program, ::rust::Slice<::rust::Str const> varyings, ::std::uint32_t buffer_mode, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl2_uniform1ui(::std::int32_t location, ::std::uint32_t v0, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl2_uniform1uiv(::std::int32_t location, ::rust::Slice<::std::uint32_t const> data, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl2_uniform2ui(::std::int32_t location, ::std::uint32_t v0, ::std::uint32_t v1, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl2_uniform2uiv(::std::int32_t location, ::rust::Slice<::std::uint32_t const> data, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl2_uniform3ui(::std::int32_t location, ::std::uint32_t v0, ::std::uint32_t v1, ::std::uint32_t v2, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl2_uniform3uiv(::std::int32_t location, ::rust::Slice<::std::uint32_t const> data, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl2_uniform4ui(::std::int32_t location, ::std::uint32_t v0, ::std::uint32_t v1, ::std::uint32_t v2, ::std::uint32_t v3, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl2_uniform4uiv(::std::int32_t location, ::rust::Slice<::std::uint32_t const> data, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl2_uniform_block_binding(::std::uint32_t program, ::std::uint32_t uniform_block_index, ::std::uint32_t uniform_block_binding, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl2_uniform_matrix2x3fv(::std::int32_t location, bool transpose, ::rust::Slice<float const> data, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl2_uniform_matrix2x4fv(::std::int32_t location, bool transpose, ::rust::Slice<float const> data, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl2_uniform_matrix3x2fv(::std::int32_t location, bool transpose, ::rust::Slice<float const> data, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl2_uniform_matrix3x4fv(::std::int32_t location, bool transpose, ::rust::Slice<float const> data, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl2_uniform_matrix4x2fv(::std::int32_t location, bool transpose, ::rust::Slice<float const> data, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl2_uniform_matrix4x3fv(::std::int32_t location, bool transpose, ::rust::Slice<float const> data, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl2_vertex_attrib_divisor(::std::uint32_t index, ::std::uint32_t divisor, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl2_vertex_attrib_i4i(::std::uint32_t index, ::std::int32_t x, ::std::int32_t y, ::std::int32_t z, ::std::int32_t w, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl2_vertex_attrib_i4iv(::std::uint32_t index, ::rust::Slice<::std::int32_t const> value, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl2_vertex_attrib_i4ui(::std::uint32_t index, ::std::uint32_t x, ::std::uint32_t y, ::std::uint32_t z, ::std::uint32_t w, ::org::nativescript::canvas::WebGLState &state) noexcept;

void canvas_native_webgl2_vertex_attrib_i4uiv(::std::uint32_t index, ::rust::Slice<::std::uint32_t const> value, ::org::nativescript::canvas::WebGLState &state) noexcept;
} // namespace canvas
} // namespace nativescript
} // namespace org
