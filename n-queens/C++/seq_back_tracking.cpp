#include<bits/stdc++.h>
using namespace std;
#define ll long long

ll count(vector<int>& row, int n, ll now, ll cnt);
bool check(vector<int>& row, int n);
ll n_queens(int n);

int main(){
	int n;
	cin >> n;
	cout << n_queens(n) << endl;
	return 0;
}

ll n_queens(int n){
	vector<int> row(n, -1);
	return count(row, n, 0, 0);
}

bool check(vector<int>& row, int n){
	for(int i = 0; i < n; ++i){
		if(row[i] == -1){
			continue;
		}else{
			for(int j = 0; j < i; ++j){
				if(row[j]==-1)
					continue;
				if(row[i]==row[j]||abs(row[i]-row[j])==(i-j))
					return false;
			}
		}
	}
	return true;
}

ll count(vector<int>& row, int n, ll now, ll cnt){
	if (!check(row, now))
		return cnt;
	ll cnt_ = 0;
	for(int i = 0; i < n; ++i){
		row[now] = i;
		if(now+1==n){
			if(check(row, now+1)){
				cnt_++;
			}
		}else{
			cnt_ += count(row, n, now+1, cnt);
		}
	}
	return cnt + cnt_;
}