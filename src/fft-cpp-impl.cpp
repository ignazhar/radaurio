/* Ignut
started: 27.12.2023
now: 27.12.2023
████████████████████████████████████████████████████████████████████
████████████████████████████████    ████████████████████████████████
██████████████████████████████        ██████████████████████████████
██████      ██████████████████        ██████████████████      ██████
██████          ██████████████        ██████████████          ██████
██████      ██    ████████████        ████████████    ██      ██████
██████      ████    ██████████        ██████████    ████      ██████
██████      ████      ██████████    ██████████      ████      ██████
██████      ████      ██████████    ██████████    ██████      ██████
██████      ██████    ██████████    ██████████    ██████      ██████
██████      ██████    ████████        ████████    ██████      ██████
██████      ██████      ██████        ██████      ██████      ██████
██████      ████        ████            ████        ████      ██████
██████            ██████████    ████    ██████████            ██████
██████      ██      ██████    ████████    ██████      ██      ██████
██████      ██████            ████████            ██████      ██████
██████                    ██            ██                    ██████
██████████████████████      ████    ████      ██████████████████████
████████████████████████      ██    ██      ████████████████████████
██████████████████████████                ██████████████████████████
██████████████████████████████        ██████████████████████████████
████████████████████████████████████████████████████████████████████
*/

#include <bits/stdc++.h>

using namespace std;
using ll = long long;

const long double PI = acos(-1);

void FFT(vector<complex<long double>> &p, complex<long double> w) {
    int n = p.size();
    if (n == 1)
        return;
    
    vector<complex<long double>> a(n / 2), b(n / 2);
    for (int i = 0; i < n / 2; i ++) {
        a[i] = p[i * 2];
        b[i] = p[i * 2 + 1];
    }
    
    FFT(a, w * w);
    FFT(b, w * w);

    complex<long double> ww = 1;
    for (int i = 0; i < n / 2; i++) {
        p[i] = a[i] + ww * b[i];
        p[i + n / 2] = a[i] - ww * b[i]; // w^(i+n/2) = -w^i
        ww *= w;
    }
}

void Multiply(vector<int> a, vector<int> b, vector<ll> &res) {
    vector<complex<long double>> fa, fb;
    for (int val : a) fa.push_back(val);
    for (int val : b) fb.push_back(val);
    int n = 1;
    while (n < max(fa.size(), fb.size()))
        n <<= 1;
    n <<= 1;
    fa.resize(n, 0), fb.resize(n, 0);

    FFT(fa, polar((long double)1., 2 * PI / n));
    FFT(fb, polar((long double)1., 2 * PI / n));

    vector<complex<long double>> c(n);
    for (int i = 0; i < n; i ++) c[i] = fa[i] * fb[i];

    // interpolate

    FFT(c, polar((long double)1., -2 * PI / n));

    res.resize(n);
    for (int i = 0; i < n; i ++)
        res[i] = round(real(c[i]) / n);
}

int main()
{
    ios_base::sync_with_stdio(false), cin.tie(0), cout.tie(0);

#ifndef ONLINE_JUDGE
    freopen("input123.txt", "r", stdin);
    freopen("output123.txt", "w", stdout);
#endif

    int n, m;
    cin >> n >> m;
    vector<int> a(n + 1), b(m + 1);
    vector <ll> res(n + m + 1);
    for (int i = 0; i <= n; i ++) cin >> a[i];
    for (int i = 0; i <= m; i ++) cin >> b[i];
    Multiply(a, b, res);
    cout << n + m << '\n';
    for (int i = 0; i < n + m + 1; i ++) cout << res[i] << ' ';
    cout << '\n';
    return 0;
}

/*
*/