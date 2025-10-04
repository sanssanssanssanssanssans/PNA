#include <bits/stdc++.h>
struct Value {
  std::variant<std::monostate,double,std::string,bool,std::map<std::string,Value>> v;
  Value():v(std::monostate{}){}
  Value(double d):v(d){}
  Value(const char*s):v(std::string(s)){}
  Value(const std::string&s):v(s){}
  Value(bool b):v(b){}
  Value(std::map<std::string,Value> o):v(std::move(o)){}
  template<class T, class = std::enable_if_t<std::is_integral_v<T> && !std::is_same_v<T,bool>>>
  Value(T x):v(static_cast<double>(x)){}
};

struct Env{ std::unordered_map<std::string,Value> vars; Env* parent=nullptr; };

inline bool is_num(const Value& x){ return std::holds_alternative<double>(x.v); }
inline bool is_str(const Value& x){ return std::holds_alternative<std::string>(x.v); }
inline bool is_bool(const Value& x){ return std::holds_alternative<bool>(x.v); }
inline bool is_obj(const Value& x){ return std::holds_alternative<std::map<std::string,Value>>(x.v); }


inline std::string as_str(const Value& x){
  if(is_str(x)) return std::get<std::string>(x.v);
  if(is_num(x)){ std::ostringstream oss; oss<<std::get<double>(x.v); return oss.str(); }
  if(is_bool(x)) return std::get<bool>(x.v)?"true":"false";
  if(is_obj(x)){
    std::string s="{"; bool first=true;
    for(auto&kv:std::get<std::map<std::string,Value>>(x.v)){
      if(!first)s+=", "; first=false; s+=kv.first+":"+as_str(kv.second);
    }
    s+="}";
    return s;
  }
  return "";
}
inline double as_num(const Value& v){
    if (is_num(v))  return std::get<double>(v.v);
    if (is_bool(v)) return std::get<bool>(v.v) ? 1.0 : 0.0;
    if (is_str(v)) {
        const auto& s = std::get<std::string>(v.v);
        char* end = nullptr;
        double d = std::strtod(s.c_str(), &end);
        if (end != s.c_str() && *end == '\0') return d; 
        return 0.0;
    }
    return 0.0;
}
inline bool truthy(const Value& x){
  if(is_bool(x)) return std::get<bool>(x.v);
  if(is_num(x)) return std::get<double>(x.v)!=0;
  if(is_str(x)) return !std::get<std::string>(x.v).empty();
  if(is_obj(x)) return !std::get<std::map<std::string,Value>>(x.v).empty();
  return false;
}

inline Value pna_get(Env& e,const std::string& k){ auto it=e.vars.find(k); if(it!=e.vars.end()) return it->second; if(e.parent) return pna_get(*e.parent,k); return Value(); }
inline void  pna_set(Env& e,const std::string& k, const Value& v){ e.vars[k]=v; }

inline Value pna_make_obj(){ return Value(std::map<std::string,Value>{}); }
inline void  pna_obj_set(Value& o,const std::string& k,const Value& v){
  if(!is_obj(o)) o=std::map<std::string,Value>{};
  std::get<std::map<std::string,Value>>(o.v)[k]=v;
}

inline Value pna_get_prop(const Value& o, const std::string& k){
  if(!is_obj(o)) return Value();
  auto &m=std::get<std::map<std::string,Value>>(const_cast<Value&>(o).v);
  auto it=m.find(k); if(it!=m.end()) return it->second; return Value();
}
inline void  pna_set_prop(Env& e,const std::string& base,const std::string& key,const Value& v){
  Value b=pna_get(e,base);
  if(!is_obj(b)) b=std::map<std::string,Value>{};
  auto m=std::get<std::map<std::string,Value>>(b.v);
  m[key]=v; b=Value(m);
  pna_set(e,base,b);
}

inline Value pna_input(const char* prompt) {
    if (prompt && prompt[0] != '\0') { std::cout << prompt; std::cout.flush(); }
    std::string s;
    if (!(std::cin >> s)) s = "";
    char* end = nullptr;
    double d = std::strtod(s.c_str(), &end);
    if (end != s.c_str() && *end == '\0') {
        return Value(d);
    }
    return Value(s);
}


inline void pna_log(const Value& v) {
    std::cout << as_str(v) << '\n';
}

inline Value pna_add(const Value&a,const Value&b){ if(is_str(a)||is_str(b)) return Value(as_str(a)+as_str(b)); return Value(as_num(a)+as_num(b)); }
inline Value pna_sub(const Value&a,const Value&b){ return Value(as_num(a)-as_num(b)); }
inline Value pna_mul(const Value& a, const Value& b){
    if (is_str(a) && is_num(b)) {
        const std::string& s = std::get<std::string>(a.v);
        long long rll = (long long)std::llround(as_num(b));
        int r = (rll < 0) ? 0 : (int)rll; 
        std::string out; out.reserve(s.size() * (size_t)std::max(r,0));
        for(char c : s){
            for(int i=0;i<r;++i) out.push_back(c);
        }
        return Value(out);
    }
    if (is_num(a) && is_str(b)) {
        return pna_mul(b, a);
    }
    return Value(as_num(a)*as_num(b));
}

inline Value pna_div(const Value&a,const Value&b){ double r=as_num(b); return Value(r==0.0?0.0:as_num(a)/r); }
inline Value pna_mod(const Value&a,const Value&b){
    double x = as_num(a), y = as_num(b);
    if (y == 0.0) return Value(0.0);
    double q = std::floor((x / y) + 1e-12);
    double r = x - q * y;
    if (std::fabs(r) < 1e-12) r = 0.0;
    return Value(r);
}
inline Value pna_eq(const Value&a,const Value&b){
    if (is_num(a) && is_num(b)) return Value(as_num(a) == as_num(b));
    return Value(as_str(a) == as_str(b));
}
inline Value pna_neq(const Value&a,const Value&b){
    if (is_num(a) && is_num(b)) return Value(as_num(a) != as_num(b));
    return Value(as_str(a) != as_str(b));
}
inline Value pna_lt (const Value&a,const Value&b){ return Value(as_num(a)< as_num(b)); }
inline Value pna_le (const Value&a,const Value&b){ return Value(as_num(a)<=as_num(b)); }
inline Value pna_gt (const Value&a,const Value&b){ return Value(as_num(a)> as_num(b)); }
inline Value pna_ge (const Value&a,const Value&b){ return Value(as_num(a)>=as_num(b)); }
inline Value pna_and(const Value&a,const Value&b){ return Value(truthy(a) && truthy(b)); }
inline Value pna_or (const Value&a,const Value&b){ return Value(truthy(a) || truthy(b)); }
inline Value pna_not(const Value&a){ return Value(!truthy(a)); }
inline bool  pna_truthy(const Value& v){ return truthy(v); }