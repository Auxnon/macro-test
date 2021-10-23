ply={x=0,y=0,vx=0,vy=0}
speed=0.2
moving=false
stance=0
jump_count=0
jump_once=false

text=0
once=true
alt=0
anim_delay=0

function _init()
for i=0,15 do
	for j=32,47 do
		if(sget(i,j)!=0)then
		blk(i*8,(j-32)*8)
		end
	end
end
end

function _update60()
	--test_obj[3]=depth-20
	test_blocks()
	moving=false
	if(#hit_test!=-2)then
		
		spd=speed
		if(stance!=1)then
		spd=speed/4
		end
		
		if (btn(0) and ply.vx > -1) ply.vx -= spd moving=true
	 if (btn(1) and ply.vx<1 ) ply.vx += spd moving=true
	 
	 if (btn(2)) then
		 if(jump_once) then
		  jump_once=false
		  if(stance==1)then
		  	stance=0
		  	ply.vy=-2
		  	ply.y-=1
		  	hit_test={}
		  	prt(13,ply.x,ply.y,1,3)
		  elseif(stance==2)then
		  	stance=0
		  	prt(13,ply.x,ply.y,1.2,2)
		  	if(ply.vx>0)	then 
		  	ply.vx=-1
		  	else 
		  	ply.vx=1
		  	end
		  	
		  	ply.x+=ply.vx*2
		  	ply.vy=-1.5
		  	hit_test={}
		  elseif(jump_count>0)then
		  prt(14,ply.x,ply.y,0.2,5)
		  ply.vy=-0.9
		  ply.vx*=0.8
		  jump_count-=1
		  end
	  end
	  --ply.y -= speed moving=true
	 else
	 	jump_once=true
	 end
	 
	 if (btn(3) and ply.y < 128) ply.y += speed moving=true
		if(	btn(4)) then
			if(once) then
				blk(ply.x,ply.y-16)
				once=false
			end
		else
		once=true
		end
			
	end
	if(#hit_test>0)then
	foreach(hit_test,function(hit) 
	collide(blocks[hit]) 
	end)
	end
	
	if(ply.y<120) then

	if(ply.vy<8)ply.vy+=0.1
	ply.y+=ply.vy
	else
	ply.vy=0
	ply.y=120
	reset_jump()
	end
	
	ply.x+=ply.vx
	if(stance!=0) then
		if(abs(ply.vx)>0.1) then
			ply.vx*=0.8
		else
			ply.vx=0
		end
	end
	
	
	anim_delay+=1
	if(anim_delay>2) alt,anim_delay=alt==1 and 0 or 1,0
	
end

function _draw()
	cls()
	draw_blocks()
	spr(1,ply.x,ply.y) --17+alt
	draw_particles()
	print('col '..#hit_test,0,0)
	print('x_ '..text,0,12)
end

function collide(b)
		x=(ply.x-b[1])
		y=(ply.y-b[2])
		text=x
		if(abs(x)>abs(y))then
			if(x>0)then
			ply.x=b[1]+8
			else
			ply.x=b[1]-8
			end
			if(stance!=1) stance=2
		else
		ply.vy=0
			if(y>0)then
			ply.y=b[2]+8
			else
			ply.y=b[2]-8
			reset_jump()
			end
		end
		
end

function reset_jump()
		stance=1
			jump_count=1
end