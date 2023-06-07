#include <iostream>
#include <limits>
#include <map>

template <typename K, typename V> class MyMap {
  V _valBegin;
  std::map<K, V> _map;

public:
  MyMap(V const &val) : _valBegin(val) {}
  void print() {
    for (const auto &[key, value] : this->_map)
      std::cout << '[' << key << "] = " << value << "; ";
    std::cout << std::endl;
  }
  void assign(K const &keyBegin, K const &keyEnd, V const &val) {
    if (_map.size() == 0) {
      _map.emplace(std::numeric_limits<K>::min(), _valBegin);
    }
    if (!(keyBegin < keyEnd) || keyBegin <= std::numeric_limits<K>::min() ||
        keyEnd > std::numeric_limits<K>::max()) {
      return;
    }

    auto it_low = _map.lower_bound(keyBegin);
    auto it_upp = _map.upper_bound(keyEnd);

    --it_upp;
    auto whatEnds = it_upp->second;
    ++it_upp;

    _map.erase(it_low, it_upp);
    _map.emplace(keyBegin, val);
    _map.emplace(keyEnd, whatEnds);
  }

  void make_canonical() {
    V current, previous;
    for (auto it = _map.begin(); it != _map.end();) {
      previous = current;
      current = it->second;
      if (current == previous) {
        _map.erase(it++);
      } else {
        ++it;
      }
    }
  }
};

int main() {
  MyMap<int, char> m('_');
  m.assign(-5, 0, 'A');
  m.assign(0, 5, 'B');
  m.assign(5, 10, 'A');
  m.print();
}

/*
 m.assign(3, 8, 'A');
  m.print();
  m.assign(10, 18, 'C');
  m.print();
  m.assign(0, 10, 'D');
  m.print();
  m.assign(2, 5, 'C');
  m.print();
  m.assign(5, 10, 'F');
  m.print();
  m.assign(-10, 0, 'P');
  m.print();
  m.assign(20, 25, 'C');
  m.print();
  m.assign(-30, -20, 'O');
  m.print();
  m.assign(-35, 35, 'X');
  m.print();
  m.assign(40, 45, 'X');
  m.print();
  m.assign(50, 55, 'X');
  m.print();
  m.assign(-100, 50, 'B');
  m.print();
  m.assign(50, 100, 'B');
  m.print();
  m.assign(100, 150, 'B');
  m.print();
  m.assign(200, 250, 'B');
  m.print();
  m.assign(250, 300, 'C');
  m.print();
  m.assign(-200, 250, 'B');
  m.print();
  m.make_canonical();
  m.print();
*/
